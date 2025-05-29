use {
    crate::discriminator::AccountDiscriminator,
    pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult},
};

/// The AccountSerialize trait is used to handle serialization of accounts
pub trait AccountSerialize: AccountDiscriminator {
    /// Defines the serialized size of the account (fields + discriminator)
    const SERIALIZED_SIZE: usize;

    /// Serializes the struct, prefixed with the discriminator
    ///
    /// Used for off-chain/testing
    fn to_bytes(&self) -> Result<Vec<u8>, ProgramError> {
        let mut data = vec![0u8; Self::SERIALIZED_SIZE];

        self.into_bytes(&mut data)?;

        Ok(data)
    }

    /// Similar to [`AccountSerialize::to_bytes`], but avoids vec allocations
    ///
    /// Intended for use with on-chain serialization
    fn into_bytes(&self, buffer: &mut [u8]) -> Result<(), ProgramError> {
        if buffer.len() < Self::SERIALIZED_SIZE {
            return Err(ProgramError::AccountDataTooSmall);
        }

        buffer[0] = Self::DISCRIMINATOR;
        buffer[1..].copy_from_slice(&self.to_bytes_inner());

        Ok(())
    }

    /// Serializes the struct, without the discriminator prefix
    fn to_bytes_inner(&self) -> Vec<u8>;
}

/// The AccountDeserialize trait is used to handle serialization of data into types
pub trait AccountDeserialize: AccountDiscriminator + Sized {
    /// Deserializes the given bytes, first validating that the discriminator matches
    fn try_from_bytes(data: &[u8]) -> Result<Self, ProgramError> {
        if data[0] != Self::DISCRIMINATOR {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(Self::from_bytes(&data[1..]))
    }
    fn from_bytes(data: &[u8]) -> Self;
}

/// The AccountWrite trait is used to handle persisting accounts into [`AccountInfo`]
pub trait AccountWrite: AccountSerialize {
    /// Writes the serialized account (with discriminator)
    fn account_write(&self, account_info: &AccountInfo) -> ProgramResult {
        let mut data = account_info.try_borrow_mut_data()?;

        self.account_write_into(&mut data[..Self::SERIALIZED_SIZE])
    }

    fn account_write_into(&self, buffer: &mut [u8]) -> Result<(), ProgramError> {
        self.into_bytes(buffer)
    }
}

#[cfg(test)]
mod test {
    use {
        super::*,
        crate::{account::AccountDeserialize, uint::parse_u64},
        pinocchio::pubkey::Pubkey,
    };

    #[derive(Debug, PartialEq, Eq)]
    pub struct FooBar {
        pub key: Pubkey,
        pub amount: u64,
    }

    impl AccountDiscriminator for FooBar {
        const DISCRIMINATOR: u8 = 69;
    }

    impl AccountSerialize for FooBar {
        const SERIALIZED_SIZE: usize = 1 // discriminator
            + 32 // key
            + 8; // amount

        fn to_bytes_inner(&self) -> Vec<u8> {
            let mut buf = Vec::with_capacity(40);

            buf.extend_from_slice(&self.key);
            buf.extend_from_slice(&self.amount.to_le_bytes());

            buf
        }
    }

    impl AccountDeserialize for FooBar {
        fn from_bytes(data: &[u8]) -> Self {
            let key: Pubkey = data[0..32].try_into().expect("insufficient bytes");
            let amount = parse_u64(&data[32..]);

            Self { key, amount }
        }
    }

    impl AccountWrite for FooBar {}

    #[test]
    fn test_account_serialize() {
        let foo_bar = FooBar {
            key: pinocchio_pubkey::from_str("9yyz5BqahoXPivcGdBKpgqt5dbTTLELNW8LkPRwWagqs"),
            amount: 420_69_1337_1234,
        };

        let foo_bar_bytes = foo_bar.to_bytes().unwrap();

        let decoded_foobar = FooBar::try_from_bytes(&foo_bar_bytes).unwrap();

        assert_eq!(foo_bar, decoded_foobar);

        assert_eq!(
            bs58::encode(&decoded_foobar.key).into_string().as_str(),
            "9yyz5BqahoXPivcGdBKpgqt5dbTTLELNW8LkPRwWagqs"
        );

        assert_eq!(decoded_foobar.amount, 420_69_1337_1234);
    }

    #[test]
    fn test_account_write() {
        let foo_bar = FooBar {
            key: pinocchio_pubkey::from_str("9yyz5BqahoXPivcGdBKpgqt5dbTTLELNW8LkPRwWagqs"),
            amount: 420_69_1337_1234,
        };
        let mut buffer = [0u8; FooBar::SERIALIZED_SIZE];

        foo_bar.account_write_into(&mut buffer).unwrap();

        let decoded_foobar = FooBar::try_from_bytes(&buffer).unwrap();

        assert_eq!(foo_bar, decoded_foobar);

        assert_eq!(
            bs58::encode(&decoded_foobar.key).into_string().as_str(),
            "9yyz5BqahoXPivcGdBKpgqt5dbTTLELNW8LkPRwWagqs"
        );

        assert_eq!(decoded_foobar.amount, 420_69_1337_1234);
    }

    #[test]
    #[should_panic(expected = "InvalidAccountData")]
    fn test_account_deserialize_invalid_discriminator() {
        FooBar::try_from_bytes(&[4, 2, 0]).unwrap();
    }
}
