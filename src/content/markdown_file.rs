//! [`MarkdownFile`].

/// A downloaded Markdown document: the repository file name plus its raw
/// Markdown source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarkdownFile {
    /// File name within the listed directory (e.g. `welcome.md`).
    pub name: String,
    /// Raw Markdown source.
    pub markdown: String,
}
