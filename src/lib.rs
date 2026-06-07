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
///
/// The four fields are framed unambiguously (each is length-prefixed) before
/// hashing, so no field value can ever forge a delimiter and collide with a
/// different `(provider, model, dim, text)` tuple. This upholds the
/// "no false hits" guarantee even for inputs containing newlines, `=`, or
/// other separator-like bytes.
pub fn key(provider: &str, model: &str, dim: usize, text: &str) -> String {
    let mut buf = Vec::with_capacity(provider.len() + model.len() + text.len() + 64);
    push_field(&mut buf, provider.as_bytes());
    push_field(&mut buf, model.as_bytes());
    push_field(&mut buf, dim.to_string().as_bytes());
    push_field(&mut buf, text.as_bytes());
    sha256::hex(&buf)
}

/// Append `field` to `buf` with an unambiguous length prefix.
///
/// Writes the field's byte length as a fixed 8-byte big-endian integer
/// followed by the raw bytes. Because the length is encoded out-of-band,
/// the field content cannot be confused with a delimiter, so distinct
/// field tuples always produce distinct byte sequences.
fn push_field(buf: &mut Vec<u8>, field: &[u8]) {
    buf.extend_from_slice(&(field.len() as u64).to_be_bytes());
    buf.extend_from_slice(field);
}
