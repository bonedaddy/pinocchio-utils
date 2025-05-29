//! Trait for defining instruction processing functions

use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

/// InstructionProcessor is a trait used to define how any given instruction may be constructed, and subsequently processed
pub trait InstructionProcessor<'a, Ix>: Sized {
    /// Constructions the instruction process type from a slice of accounts
    fn from_accounts(accounts: &'a [AccountInfo]) -> Result<Self, ProgramError>;

    fn try_process(&self, instruction: Ix) -> ProgramResult {
        self.validations(&instruction)?;
        self.process(instruction)
    }

    /// Handler function which invocations the business logic for an instruction
    fn process(&self, instruction: Ix) -> ProgramResult;

    /// Validations which which performs validation of the instruction inputs, and acounts
    fn validations(&self, instruction: &Ix) -> ProgramResult;
}

#[cfg(test)]
mod test {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum TestInstruction {
        Hello { msg: Vec<u8> },
        Init { force: bool },
    }
    pub struct HelloAccounts<'a> {
        payer: &'a AccountInfo,
        msg: &'a AccountInfo,
        system_program: &'a AccountInfo,
    }

    impl<'a> TryFrom<&'a [AccountInfo]> for HelloAccounts<'a> {
        type Error = ProgramError;

        fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
            let [payer, msg, system_program] = accounts else {
                return Err(ProgramError::NotEnoughAccountKeys);
            };

            Ok(Self {
                payer,
                msg,
                system_program,
            })
        }
    }

    impl<'a> InstructionProcessor<'a, TestInstruction> for HelloAccounts<'a> {
        fn from_accounts(accounts: &'a [AccountInfo]) -> Result<Self, ProgramError> {
            HelloAccounts::try_from(accounts)
        }
        fn process(&self, instruction: TestInstruction) -> ProgramResult {
            Ok(())
        }
        fn validations(&self, instruction: &TestInstruction) -> ProgramResult {
            Ok(())
        }
    }
}
