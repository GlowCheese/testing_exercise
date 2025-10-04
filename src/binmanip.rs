#[derive(Debug, PartialEq, Eq)]
pub enum BinPowError {
    InvalidInput,
    Indeterminate,
}

pub fn binpow(mut a: i64, mut b: i64, m: i64) -> Result<i64, BinPowError> {
    if a.abs() > 1000 || b < 0 || m < 1 || b.max(m) > 1000 {
        Err(BinPowError::InvalidInput)
    } else if a == 0 && b == 0 {
        Err(BinPowError::Indeterminate)
    } else {
        let mut result = 1;
        while b > 0 {
            if b & 1 == 1 {
                result = result * a % m;
            }
            b >>= 1;
            a = a * a % m;
        }
        Ok(result)
    }
}
