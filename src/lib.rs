use std::fmt::Display;

pub fn zero_pad<T: Display>(num: T, min_length: usize) -> String {
    format!("{:0>width$}", num, width = min_length)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_digit() {
        assert_eq!(zero_pad(1, 6), "000001");
    }

    #[test]
    fn two_digits() {
        assert_eq!(zero_pad(23, 6), "000023");
    }

    #[test]
    fn no_need() {
        assert_eq!(zero_pad(123456, 6), "123456");
    }

    #[test]
    fn extra() {
        assert_eq!(zero_pad(1234567, 6), "1234567");
    }
}
