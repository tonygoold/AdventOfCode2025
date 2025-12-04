pub fn num_digits(val: u64) -> u32 {
    val.ilog10() + 1
}

pub fn remove_suffix(val: u64, suffix: u64) -> Option<u64> {
    let modulus = 10u64.pow(num_digits(suffix));
    if val % modulus == suffix {
        Some(val / modulus)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_digits_single() {
        let result = num_digits(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_num_digits_999() {
        let result = num_digits(999);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_suffix_good() {
        let result = remove_suffix(1299, 99);
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_suffix_bad() {
        let result = remove_suffix(1298, 99);
        assert!(result.is_none());
    }
}
