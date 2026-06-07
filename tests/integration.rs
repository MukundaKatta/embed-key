use embed_key::key;

#[test]
fn returns_64_hex() {
    let k = key("openai", "text-embedding-3-large", 3072, "hello");
    assert_eq!(k.len(), 64);
    assert!(k.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn text_change_changes_key() {
    let a = key("openai", "m", 1, "a");
    let b = key("openai", "m", 1, "b");
    assert_ne!(a, b);
}

#[test]
fn provider_change_changes_key() {
    let a = key("openai", "m", 1, "x");
    let b = key("anthropic", "m", 1, "x");
    assert_ne!(a, b);
}

#[test]
fn model_change_changes_key() {
    let a = key("p", "v1", 1, "x");
    let b = key("p", "v2", 1, "x");
    assert_ne!(a, b);
}

#[test]
fn dim_change_changes_key() {
    let a = key("p", "m", 768, "x");
    let b = key("p", "m", 1536, "x");
    assert_ne!(a, b);
}

#[test]
fn identical_inputs_identical_keys() {
    let a = key("p", "m", 1, "x");
    let b = key("p", "m", 1, "x");
    assert_eq!(a, b);
}

#[test]
fn empty_inputs_are_valid() {
    let k = key("", "", 0, "");
    assert_eq!(k.len(), 64);
    assert!(k.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn delimiter_like_content_does_not_collide() {
    // A field that contains separator-looking bytes must never collide with a
    // different field split. With naive `\n`-joined framing these two distinct
    // tuples hashed to the same pre-image; unambiguous framing keeps them apart.
    let a = key("openai\nm=evil", "real", 1, "x");
    let b = key("openai", "evil\nm=real", 1, "x");
    assert_ne!(a, b);

    let c = key("a", "bc", 1, "x");
    let d = key("ab", "c", 1, "x");
    assert_ne!(c, d);
}
