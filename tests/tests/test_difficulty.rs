use pow_core::hash::{validate_hash, zeros_amount};

#[test]
fn test_leading_zeros() {
    assert_eq!(
        zeros_amount("00000000fda584dabcd5688a20d2c1ddc4aac85493a2b1b47097f167e283a362"),
        8 * 4
    );

    assert_eq!(zeros_amount("0007"), 3 * 4 + 1);
    assert_eq!(zeros_amount("0008"), 3 * 4);
    assert_eq!(zeros_amount("88"), 0);
}

#[test]
fn test_difficulty() {
    assert!(validate_hash(
        "00acc6dd1da584dabcd5688a20d2c1ddc4aac85493a2b1b47097f167e283a362",
        1.0
    ));
    assert!(validate_hash(
        "00acc6dd1da584dabcd5688a20d2c1ddc4aac85493a2b1b47097f167e283a362",
        256.0
    ));
    assert!(validate_hash(
        "00acc6dd1da584dabcd5688a20d2c1ddc4aac85493a2b1b47097f167e283a362",
        511.0
    ));
    assert!(!validate_hash(
        "00acc6dd1da584dabcd5688a20d2c1ddc4aac85493a2b1b47097f167e283a362",
        512.0
    ));

    assert!(validate_hash(
        "00000000fda584dabcd5688a20d2c1ddc4aac85493a2b1b47097f167e283a362",
        2.0f64.powi(4 * 8 + 1) - 0.01
    ));
    assert!(!validate_hash(
        "00000000fda584dabcd5688a20d2c1ddc4aac85493a2b1b47097f167e283a362",
        2.0f64.powi(4 * 8 + 1)
    ));
}
