//! # embed-key
//!
//! Deterministic 64-char cache key for an embedding request.
//!
//! Mixes the text with `(provider, model, dimensionality)` so a single
//! shared cache can hold embeddings from multiple providers/models
//! without false hits.
//!
//! ## Example
//!
//! ```
//! use embed_key::key;
//! let k = key("openai", "text-embedding-3-large", 3072, "hello world");
//! assert_eq!(k.len(), 64);
//! ```

#![deny(missing_docs)]

mod sha256;

/// Build a 64-char hex cache key.
pub fn key(provider: &str, model: &str, dim: usize, text: &str) -> String {
    let mut buf = String::with_capacity(text.len() + 64);
    buf.push_str("p=");
    buf.push_str(provider);
    buf.push('\n');
    buf.push_str("m=");
    buf.push_str(model);
    buf.push('\n');
    buf.push_str("d=");
    buf.push_str(&dim.to_string());
    buf.push('\n');
    buf.push_str("t=");
    buf.push_str(text);
    sha256::hex(buf.as_bytes())
}
