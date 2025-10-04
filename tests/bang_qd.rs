use my_testing_exercise::binmanip::{BinPowError, binpow};

#[test]
fn test_1() {
    assert_eq!(binpow(98, 500, -1), Err(BinPowError::InvalidInput));
}

#[test]
fn test_2() {
    assert_eq!(binpow(-1001, 500, 500), Err(BinPowError::InvalidInput));
}

#[test]
fn test_3() {
    assert_eq!(binpow(-98, -1, 500), Err(BinPowError::InvalidInput));
}

#[test]
fn test_4() {
    assert_eq!(binpow(-98, 0, 500), Ok(1));
}

#[test]
fn test_5() {
    assert_eq!(binpow(-98, 500, 500), Ok(376));
}

#[test]
fn test_6() {
    assert_eq!(binpow(-98, 501, 500), Ok(-348));
}

#[test]
fn test_7() {
    assert_eq!(binpow(-98, 1001, 500), Err(BinPowError::InvalidInput));
}

#[test]
fn test_8() {
    assert_eq!(binpow(0, -1, 500), Err(BinPowError::InvalidInput));
}

#[test]
fn test_9() {
    assert_eq!(binpow(0, 0, 500), Err(BinPowError::Indeterminate));
}

#[test]
fn test_10() {
    assert_eq!(binpow(0, 500, 500), Ok(0));
}

#[test]
fn test_11() {
    assert_eq!(binpow(0, 501, 500), Ok(0));
}

#[test]
fn test_12() {
    assert_eq!(binpow(0, 1001, 500), Err(BinPowError::InvalidInput));
}

#[test]
fn test_13() {
    assert_eq!(binpow(98, -1, 500), Err(BinPowError::InvalidInput));
}

#[test]
fn test_14() {
    assert_eq!(binpow(98, 0, 500), Ok(1));
}

#[test]
fn test_15() {
    assert_eq!(binpow(98, 500, 500), Ok(376));
}

#[test]
fn test_16() {
    assert_eq!(binpow(98, 501, 500), Ok(348));
}

#[test]
fn test_17() {
    assert_eq!(binpow(98, 1001, 500), Err(BinPowError::InvalidInput));
}

#[test]
fn test_18() {
    assert_eq!(binpow(1001, 500, 500), Err(BinPowError::InvalidInput));
}

#[test]
fn test_19() {
    assert_eq!(binpow(98, 500, 1001), Err(BinPowError::InvalidInput));
}
