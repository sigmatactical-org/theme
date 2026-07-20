//! GitHub-backed Markdown content pipeline shared by the Sigma content
//! sites: front-matter parsing, Markdown rendering, and a GitHub contents
//! listing + download client.

mod content_error;
mod github_content_entry;
mod markdown_file;
pub use content_error::ContentError;
pub use github_content_entry::GithubContentEntry;
pub use markdown_file::MarkdownFile;

use std::collections::BTreeMap;

use futures_util::{StreamExt, TryStreamExt};
use pulldown_cmark::{Options, Parser, html};

/// Number of file downloads in flight at once for [`fetch_markdown_files`].
const DOWNLOAD_CONCURRENCY: usize = 8;

/// Split an optional `---`-fenced front-matter header off a Markdown source.
///
/// Returns the `key: value` pairs from the header (empty when there is no
/// front matter) and the remaining Markdown body.
#[must_use]
pub fn split_front_matter(source: &str) -> (BTreeMap<String, String>, &str) {
    let mut meta = BTreeMap::new();
    let Some(rest) = source.strip_prefix("---") else {
        return (meta, source);
    };
    let Some((header, body)) = rest.split_once("\n---") else {
        return (meta, source);
    };
    let body = body.trim_start_matches('\n');
    for line in header.lines() {
        if let Some((key, value)) = line.split_once(':') {
            meta.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    (meta, body)
}

/// Render Markdown to HTML with the shared option set (tables and
/// strikethrough enabled).
#[must_use]
pub fn markdown_to_html(markdown: &str) -> String {
    let mut html_out = String::new();
    let parser = Parser::new_ext(
        markdown,
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH,
    );
    html::push_html(&mut html_out, parser);
    html_out
}

/// List a directory of a GitHub repository via the contents API.
///
/// `path` may be empty to list the repository root. `user_agent` identifies
/// the calling service to GitHub (required by their API).
///
/// # Errors
///
/// Returns [`ContentError`] when the request fails or GitHub responds with a
/// non-success status.
pub async fn list_repo_dir(
    client: &reqwest::Client,
    owner: &str,
    repo: &str,
    path: &str,
    git_ref: &str,
    user_agent: &str,
) -> Result<Vec<GithubContentEntry>, ContentError> {
    let list_url =
        format!("https://api.github.com/repos/{owner}/{repo}/contents/{path}?ref={git_ref}");
    let response = client
        .get(&list_url)
        .header("accept", "application/vnd.github+json")
        .header("user-agent", user_agent)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(ContentError::Request(format!("list {status}: {body}")));
    }

    Ok(response.json().await?)
}

/// Download the Markdown files among `entries` (typically from
/// [`list_repo_dir`]), up to [`DOWNLOAD_CONCURRENCY`] at a time.
///
/// Non-`.md` entries and entries without a download URL are skipped; results
/// keep the listing order.
///
/// # Errors
///
/// Returns [`ContentError`] when any download fails.
pub async fn download_markdown_files(
    client: &reqwest::Client,
    entries: Vec<GithubContentEntry>,
    user_agent: &str,
) -> Result<Vec<MarkdownFile>, ContentError> {
    futures_util::stream::iter(
        entries
            .into_iter()
            .filter(|entry| entry.name.ends_with(".md"))
            .filter_map(|entry| Some((entry.name, entry.download_url?)))
            .map(|(name, download_url)| async move {
                let markdown = client
                    .get(download_url)
                    .header("user-agent", user_agent)
                    .send()
                    .await?
                    .error_for_status()?
                    .text()
                    .await?;
                Ok(MarkdownFile { name, markdown })
            }),
    )
    .buffered(DOWNLOAD_CONCURRENCY)
    .try_collect()
    .await
}

/// List a GitHub repository directory and download its Markdown files
/// concurrently: [`list_repo_dir`] followed by [`download_markdown_files`].
///
/// # Errors
///
/// Returns [`ContentError`] when the listing or any download fails.
pub async fn fetch_markdown_files(
    client: &reqwest::Client,
    owner: &str,
    repo: &str,
    path: &str,
    git_ref: &str,
    user_agent: &str,
) -> Result<Vec<MarkdownFile>, ContentError> {
    let entries = list_repo_dir(client, owner, repo, path, git_ref, user_agent).await?;
    download_markdown_files(client, entries, user_agent).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_front_matter_key_values_from_body() {
        let (meta, body) = split_front_matter("---\ntitle: Welcome\norder: 1\n---\n\nHello.\n");
        assert_eq!(meta.get("title").unwrap(), "Welcome");
        assert_eq!(meta.get("order").unwrap(), "1");
        assert_eq!(body, "Hello.\n");
    }

    #[test]
    fn source_without_front_matter_passes_through() {
        let source = "No front matter here.\n";
        let (meta, body) = split_front_matter(source);
        assert!(meta.is_empty());
        assert_eq!(body, source);
    }

    #[test]
    fn unterminated_front_matter_passes_through() {
        let source = "---\ntitle: broken\n";
        let (meta, body) = split_front_matter(source);
        assert!(meta.is_empty());
        assert_eq!(body, source);
    }

    #[test]
    fn markdown_renders_tables_and_strikethrough() {
        let html = markdown_to_html("~~gone~~\n\n| a | b |\n|---|---|\n| 1 | 2 |\n");
        assert!(html.contains("<del>gone</del>"));
        assert!(html.contains("<table>"));
    }
}
