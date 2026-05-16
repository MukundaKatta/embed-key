# embed-key

[![crates.io](https://img.shields.io/crates/v/embed-key.svg)](https://crates.io/crates/embed-key)

Deterministic 64-char cache key for an embedding request.

```rust
use embed_key::key;
let k = key("openai", "text-embedding-3-large", 3072, "hello");
```

Survives model upgrades — change any of `(provider, model, dim)` and the
key changes too. Zero deps. MIT or Apache-2.0.
