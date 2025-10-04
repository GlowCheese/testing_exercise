use my_testing_exercise::binmanip::{binpow, BinPowError};

#[test]
fn test_1() {
    assert_eq!(binpow(-1000, 500, 500), Ok(0));
}

#[test]
fn test_2() {
    assert_eq!(binpow(-999, 500, 500), Ok(1));
}

#[test]
fn test_3() {
    assert_eq!(binpow(0, 500, 500), Ok(0));
}

#[test]
fn test_4() {
    assert_eq!(binpow(999, 500, 500), Ok(1));
}

#[test]
fn test_5() {
    assert_eq!(binpow(1000, 500, 500), Ok(0));
}

#[test]
fn test_6() {
    assert_eq!(binpow(0, 0, 500), Err(BinPowError::Indeterminate));
}

#[test]
fn test_7() {
    assert_eq!(binpow(0, 1, 500), Ok(0));
}

#[test]
fn test_8() {
    assert_eq!(binpow(0, 999, 500), Ok(0));
}

#[test]
fn test_9() {
    assert_eq!(binpow(0, 1000, 500), Ok(0));
}

#[test]
fn test_10() {
    assert_eq!(binpow(0, 500, 1), Ok(0));
}

#[test]
fn test_11() {
    assert_eq!(binpow(0, 500, 2), Ok(0));
}

#[test]
fn test_12() {
    assert_eq!(binpow(0, 500, 999), Ok(0));
}

#[test]
fn test_13() {
    assert_eq!(binpow(0, 500, 1000), Ok(0));
}
