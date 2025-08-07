#![allow(dead_code)]

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub struct Check;

impl Check {
    /// Checks if the given account is a signer.
    pub fn check_is_signer(account: &AccountInfo) -> ProgramResult {
        if !account.is_signer {
            msg!("Missing required signature");
            return Err(ProgramError::MissingRequiredSignature);
        }
        Ok(())
    }

    /// Checks if the given account is the system program.
    pub fn check_system_program(account: &AccountInfo) -> ProgramResult {
        if !solana_program::system_program::check_id(account.key) {
            msg!("Incorrect system program ID");
            return Err(ProgramError::IncorrectProgramId);
        }
        Ok(())
    }

    /// Checks if the given account is the SPL Token program.
    pub fn check_token_program(account: &AccountInfo) -> ProgramResult {
        if account.key != &spl_token::ID {
            msg!("Incorrect token program ID");
            return Err(ProgramError::IncorrectProgramId);
        }
        Ok(())
    }

    /// Checks if the given account is the SPL Token 2022 program.
    pub fn check_token_2022_program(account: &AccountInfo) -> ProgramResult {
        if account.key != &spl_token_2022::ID {
            msg!("Incorrect token-2022 program ID");
            return Err(ProgramError::IncorrectProgramId);
        }
        Ok(())
    }

    /// Checks if the instruction data is not empty.
    pub fn check_instr(instruction_data: &[u8]) -> ProgramResult {
        if instruction_data.is_empty() {
            msg!("Instruction data is empty");
            return Err(ProgramError::InvalidInstructionData);
        }
        Ok(())
    }

    /// Checks if the provided program ID is the expected one.
    pub fn check_program_id(program_id: &Pubkey, expected_id: &Pubkey) -> ProgramResult {
        if program_id != expected_id {
            msg!("Incorrect program ID: expected {}, got {}", expected_id, program_id);
            return Err(ProgramError::IncorrectProgramId);
        }
        Ok(())
    }
}