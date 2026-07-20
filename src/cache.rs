//! Process-wide TTL caching for data fetched at request time (GitHub
//! listings, remote content, ...).

mod ttl_cache;
pub use ttl_cache::TtlCache;
