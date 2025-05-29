//! Helper functions for working with uint types

#[inline(always)]
pub fn parse_u64(data: &[u8]) -> u64 {
    u64::from_le_bytes(data.try_into().expect("slice must be 8 bytes"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_u64() {
        let num = 420691337_u64;

        let num2 = parse_u64(&num.to_le_bytes());

        assert_eq!(num, num2);
    }

    #[test]
    #[should_panic(expected = "slice must be 8 bytes")]
    fn test_parse_u64_insufficient_length() {
        let _ = parse_u64(&[1, 2]);
    }
}
