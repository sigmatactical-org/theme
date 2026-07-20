//! [`TtlCache`].

use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::{Mutex, RwLock};

/// How long fetch retries are suppressed after a failed refresh.
const ERROR_BACKOFF: Duration = Duration::from_secs(5);

/// TTL cache of a single fetched value with single-flight refresh and a
/// short error backoff.
///
/// - **Single-flight**: concurrent misses fetch once; later callers wait on
///   the refresh lock and reuse the fresh result.
/// - **Stale fallback**: when a refresh fails but a previous value exists,
///   the stale value keeps being served.
/// - **Error backoff**: after a failed refresh, retries are suppressed for
///   [`ERROR_BACKOFF`] while a previous value exists (instead of hammering
///   the upstream on every request, or suppressing retries for a full TTL).
///   With nothing cached the error is returned and the next request retries.
pub struct TtlCache<T> {
    value: RwLock<Option<(Arc<T>, Instant)>>,
    /// Single-flight refresh guard; holds the instant of the last failed fetch.
    refresh: Mutex<Option<Instant>>,
}

impl<T> TtlCache<T> {
    /// Empty, immediately-stale cache. `const`, so it can back a `static`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            value: RwLock::const_new(None),
            refresh: Mutex::const_new(None),
        }
    }

    /// Cached value if it was fetched within `ttl`.
    async fn fresh(&self, ttl: Duration) -> Option<Arc<T>> {
        self.value
            .read()
            .await
            .as_ref()
            .filter(|(_, fetched_at)| fetched_at.elapsed() < ttl)
            .map(|(value, _)| Arc::clone(value))
    }

    /// Cached value regardless of age.
    async fn stale(&self) -> Option<Arc<T>> {
        self.value
            .read()
            .await
            .as_ref()
            .map(|(value, _)| Arc::clone(value))
    }

    /// Return the cached value, refreshing it with `fetch` when it is
    /// missing or older than `ttl`.
    ///
    /// # Errors
    ///
    /// Returns the fetch error only when there is no previously cached
    /// value to fall back to.
    pub async fn get_or_fetch<F, Fut, E>(&self, ttl: Duration, fetch: F) -> Result<Arc<T>, E>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, E>>,
    {
        if let Some(value) = self.fresh(ttl).await {
            return Ok(value);
        }

        let mut last_failure = self.refresh.lock().await;
        // Another caller may have refreshed while we waited for the lock.
        if let Some(value) = self.fresh(ttl).await {
            return Ok(value);
        }
        if last_failure.is_some_and(|at| at.elapsed() < ERROR_BACKOFF)
            && let Some(value) = self.stale().await
        {
            return Ok(value);
        }

        match fetch().await {
            Ok(value) => {
                *last_failure = None;
                let value = Arc::new(value);
                *self.value.write().await = Some((Arc::clone(&value), Instant::now()));
                Ok(value)
            }
            Err(err) => {
                *last_failure = Some(Instant::now());
                match self.stale().await {
                    Some(value) => Ok(value),
                    None => Err(err),
                }
            }
        }
    }
}

impl<T> Default for TtlCache<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    const TTL: Duration = Duration::from_secs(60);

    #[tokio::test]
    async fn serves_fresh_value_without_refetching() {
        let cache = TtlCache::new();
        let calls = AtomicUsize::new(0);
        for _ in 0..3 {
            let value = cache
                .get_or_fetch(TTL, || async {
                    calls.fetch_add(1, Ordering::SeqCst);
                    Ok::<_, String>(42)
                })
                .await
                .unwrap();
            assert_eq!(*value, 42);
        }
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn expired_value_is_refetched() {
        let cache = TtlCache::new();
        cache
            .get_or_fetch(Duration::ZERO, || async { Ok::<_, String>(1) })
            .await
            .unwrap();
        let value = cache
            .get_or_fetch(Duration::ZERO, || async { Ok::<_, String>(2) })
            .await
            .unwrap();
        assert_eq!(*value, 2);
    }

    #[tokio::test]
    async fn error_with_empty_cache_is_returned() {
        let cache: TtlCache<i32> = TtlCache::new();
        let err = cache
            .get_or_fetch(TTL, || async { Err::<i32, _>("boom".to_string()) })
            .await
            .unwrap_err();
        assert_eq!(err, "boom");
    }

    #[tokio::test]
    async fn error_after_success_serves_stale_and_backs_off() {
        let cache = TtlCache::new();
        cache
            .get_or_fetch(TTL, || async { Ok::<_, String>(7) })
            .await
            .unwrap();

        // Expired refresh fails: the stale value is served.
        let calls = AtomicUsize::new(0);
        let value = cache
            .get_or_fetch(Duration::ZERO, || async {
                calls.fetch_add(1, Ordering::SeqCst);
                Err::<i32, _>("boom".to_string())
            })
            .await
            .unwrap();
        assert_eq!(*value, 7);
        assert_eq!(calls.load(Ordering::SeqCst), 1);

        // Within the backoff window the fetch is not retried.
        let value = cache
            .get_or_fetch(Duration::ZERO, || async {
                calls.fetch_add(1, Ordering::SeqCst);
                Err::<i32, _>("boom".to_string())
            })
            .await
            .unwrap();
        assert_eq!(*value, 7);
        assert_eq!(calls.load(Ordering::SeqCst), 1, "retry suppressed by backoff");
    }

    #[tokio::test]
    async fn concurrent_misses_fetch_once() {
        static CACHE: TtlCache<u32> = TtlCache::new();
        static CALLS: AtomicUsize = AtomicUsize::new(0);

        async fn get() -> Arc<u32> {
            CACHE
                .get_or_fetch(TTL, || async {
                    CALLS.fetch_add(1, Ordering::SeqCst);
                    tokio::task::yield_now().await;
                    Ok::<_, String>(9)
                })
                .await
                .unwrap()
        }

        let (a, b, c) = tokio::join!(get(), get(), get());
        assert_eq!((*a, *b, *c), (9, 9, 9));
        assert_eq!(CALLS.load(Ordering::SeqCst), 1);
    }
}
