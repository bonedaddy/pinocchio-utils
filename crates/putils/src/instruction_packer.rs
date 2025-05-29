//! Instruction packing/unpacking traits

use {crate::discriminator::InstructionDiscriminator, pinocchio::program_error::ProgramError};

/// The InstructionPacker trait is used to handle packing/unpacking of instruction data
pub trait InstructionPacker: InstructionDiscriminator + Sized {
    /// Packs the instruction into its raw bytes
    fn pack(&self) -> Vec<u8>;
    /// Unpacks raw bytes into typed instruction data
    fn unpack(data: &[u8]) -> Result<Self, ProgramError>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum TestInstruction {
        Hello { msg: Vec<u8> },
        Init { force: bool },
    }

    impl InstructionDiscriminator for TestInstruction {
        fn discriminator(&self) -> u8 {
            match self {
                Self::Hello { .. } => 0,
                Self::Init { .. } => 1,
            }
        }
    }

    impl InstructionPacker for TestInstruction {
        fn pack(&self) -> Vec<u8> {
            match self {
                Self::Hello { msg } => {
                    let mut buf = Vec::with_capacity(1 + msg.len());
                    buf.push(self.discriminator());
                    buf.extend(msg);
                    buf
                }
                Self::Init { force } => {
                    let mut buf = Vec::with_capacity(2);
                    buf.push(self.discriminator());
                    buf.push(if *force { 1 } else { 0 });
                    buf
                }
            }
        }
        fn unpack(data: &[u8]) -> Result<Self, ProgramError> {
            let (first, rest) = data
                .split_first()
                .ok_or(ProgramError::InvalidInstructionData)?;
            match first {
                0 => Ok(Self::Hello { msg: rest.to_vec() }),
                1 => Ok(Self::Init {
                    force: rest[0] != 0,
                }),
                _ => Err(ProgramError::InvalidInstructionData),
            }
        }
    }

    #[test]
    fn test_packer_hello() {
        let hello = TestInstruction::Hello {
            msg: b"foobar".to_vec(),
        };

        let packed_hello = hello.pack();

        let unpacked_hello = TestInstruction::unpack(&packed_hello).unwrap();

        assert_eq!(hello, unpacked_hello);
    }

    #[test]
    fn test_packer_init() {
        let init = TestInstruction::Init { force: true };

        let packed_init = init.pack();

        let unpacked_init = TestInstruction::unpack(&packed_init).unwrap();

        assert_eq!(init, unpacked_init);

        let init2 = TestInstruction::Init { force: false };

        let unpacked_init = init2.pack();

        let unpacked_init2 = TestInstruction::unpack(&unpacked_init).unwrap();

        assert_eq!(init2, unpacked_init2);

        assert_ne!(init, unpacked_init2);
    }

    #[test]
    #[should_panic]
    fn test_packer_unpack_invalid_instruction() {
        let data: Vec<u8> = vec![69, 42, 0, 1, 3, 3, 7];

        TestInstruction::unpack(&data).unwrap();
    }
}
