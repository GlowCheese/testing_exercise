use my_testing_exercise::binmanip::{binpow, BinPowError};

#[test]
fn test_01() {
    assert_eq!(binpow(321, -1, 123), Err(BinPowError::InvalidInput));
}

#[test]
fn test_02() {
    assert_eq!(binpow(0, 0, 123), Err(BinPowError::Indeterminate));
}

#[test]
fn test_03() {
    assert_eq!(binpow(321, 2, 123), Ok(90));
}