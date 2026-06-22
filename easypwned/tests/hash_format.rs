use sha1::{Digest, Sha1};

fn sha1_hex_upper(pw: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(pw.as_bytes());
    let hash_raw = &hasher.finalize();
    base16ct::lower::encode_string(hash_raw).to_uppercase()
}

#[test]
fn matches_known_hibp_digest() {
    assert_eq!(
        sha1_hex_upper("password"),
        "5BAA61E4C9B93F3F0682250B6CF8331B7EE68FD8"
    );
}

#[test]
fn digest_is_40_uppercase_hex_chars() {
    let hash = sha1_hex_upper("");
    assert_eq!(hash, "DA39A3EE5E6B4B0D3255BFEF95601890AFD80709");
    assert_eq!(hash.len(), 40);
    assert!(hash
        .chars()
        .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_lowercase()));
}
