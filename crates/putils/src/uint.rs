//! Helper functions for working with uint types

use pinocchio::pubkey::Pubkey;

#[inline(always)]
pub fn parse_u64(data: &[u8]) -> u64 {
    u64::from_le_bytes(data.try_into().expect("slice must be 8 bytes"))
}

#[inline(always)]
pub fn parse_u32(data: &[u8]) -> u32 {
    u32::from_le_bytes(data.try_into().expect("slice must be 4 bytes"))
}

#[inline(always)]
pub fn parse_pubkey(data: &[u8]) -> Pubkey {
    data.try_into().expect("slice must be 32 bytes")
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
    fn test_parse_u32() {
        let num = 69420_u32;

        let num2 = parse_u32(&num.to_le_bytes());

        assert_eq!(num, num2);
    }

    #[test]
    fn test_parse_pubkey() {
        let key = [69u8; 32];

        let key2 = parse_pubkey(&key);

        assert_eq!(key, key2);
    }

    #[test]
    #[should_panic(expected = "slice must be 8 bytes")]
    fn test_parse_u64_insufficient_length() {
        let _ = parse_u64(&[1, 2]);
    }

    #[test]
    #[should_panic(expected = "slice must be 4 bytes")]
    fn test_parse_u32_insufficient_length() {
        let _ = parse_u32(&[1, 2]);
    }

    #[test]
    #[should_panic(expected = "slice must be 32 bytes")]
    fn test_parse_pubkey_insufficient_length() {
        let _ = parse_pubkey(&[1, 2]);
    }
}
