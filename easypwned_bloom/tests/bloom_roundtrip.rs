use bloomfilter::Bloom;
use easypwned_bloom::bloom::BloomWithMetadata;

#[test]
fn to_bloom_preserves_membership() {
    let present: Vec<u8> = b"0000000CAEF405439D57847A8657218C618160B2".to_vec();
    let absent: Vec<u8> = b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF".to_vec();

    let mut bloom: Bloom<Vec<u8>> = Bloom::new_for_fp_rate(1000, 0.0001);
    bloom.set(&present);

    let meta = BloomWithMetadata {
        number_of_bits: bloom.number_of_bits(),
        number_of_hash_functions: bloom.number_of_hash_functions(),
        sip_keys: bloom.sip_keys(),
        bloom: bloom.bitmap(),
    };

    let restored = meta.to_bloom();

    assert!(restored.check(&present));
    assert!(!restored.check(&absent));
}
