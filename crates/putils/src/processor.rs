//! Trait for defining instruction processing functions

use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

/// InstructionProcessor is a trait used to define how any given instruction may be constructed, and subsequently processed
pub trait InstructionProcessor<'a, Ix, T>: Sized {
    /// Constructions the instruction process type from a slice of accounts
    fn from_accounts(accounts: &'a [AccountInfo]) -> Result<Self, ProgramError>;

    /// The main entrypoint for processing this specific instruction, ensuring that
    ///
    /// * The instruction name is logged
    /// * Validations are performed
    /// * Business logic is invoked
    fn try_process(&self, instruction: Ix) -> ProgramResult {
        self.log_ix();
        let validations_result = self.validations(&instruction)?;
        self.process(instruction, validations_result)
    }

    /// Handler function which invocations the business logic for an instruction
    fn process(&self, instruction: Ix, validations_result: Option<T>) -> ProgramResult;

    /// Validations which which performs validation of the instruction inputs, and acounts
    ///
    /// The validations function allows returning an optional result, which can be fed into the [`InstructionProcessor::process`]
    /// function. This can be used for things like returning the bump seed from PDA derivation, to be reused within the process function
    /// to create an account, without having to re-derive the PDA
    ///
    fn validations(&self, instruction: &Ix) -> Result<Option<T>, ProgramError>;

    /// Logs the instruction name being invoked
    fn log_ix(&self);
}

#[cfg(test)]
mod test {
    use pinocchio::msg;

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

    pub struct HelloAccountsValidationResult {
        pub nocne: u8,
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

    impl<'a> InstructionProcessor<'a, TestInstruction, ()> for HelloAccounts<'a> {
        fn from_accounts(accounts: &'a [AccountInfo]) -> Result<Self, ProgramError> {
            HelloAccounts::try_from(accounts)
        }
        fn process(
            &self,
            instruction: TestInstruction,
            validations_result: Option<()>,
        ) -> ProgramResult {
            Ok(())
        }
        fn validations(&self, instruction: &TestInstruction) -> Result<Option<()>, ProgramError> {
            Ok(None)
        }
        fn log_ix(&self) {
            msg!("Instruction: HelloAccounts")
        }
    }
    impl<'a> InstructionProcessor<'a, TestInstruction, HelloAccountsValidationResult>
        for HelloAccounts<'a>
    {
        fn from_accounts(accounts: &'a [AccountInfo]) -> Result<Self, ProgramError> {
            HelloAccounts::try_from(accounts)
        }
        fn process(
            &self,
            instruction: TestInstruction,
            validations_result: Option<HelloAccountsValidationResult>,
        ) -> ProgramResult {
            Ok(())
        }
        fn validations(
            &self,
            instruction: &TestInstruction,
        ) -> Result<Option<HelloAccountsValidationResult>, ProgramError> {
            Ok(None)
        }
        fn log_ix(&self) {
            msg!("Instruction: HelloAccounts")
        }
    }
}
