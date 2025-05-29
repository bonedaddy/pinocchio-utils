//! Anchor-esque account and instruction discriminator

/// The AccountDiscriminator trait is used to uniquely identify accounts
pub trait AccountDiscriminator {
    const DISCRIMINATOR: u8;
}

/// The InstructionDiscriminator trait is used to uniquely identify instructions
pub trait InstructionDiscriminator {
    fn discriminator(&self) -> u8;
}

#[cfg(test)]
mod test {
    use super::*;

    pub struct FooBar {}

    impl AccountDiscriminator for FooBar {
        const DISCRIMINATOR: u8 = 1;
    }

    impl InstructionDiscriminator for FooBar {
        fn discriminator(&self) -> u8 {
            69
        }
    }

    #[test]
    fn test_discriminator() {
        assert_eq!(1, FooBar::DISCRIMINATOR);
        assert_eq!(69, FooBar {}.discriminator());
    }
}
