#![allow(dead_code)]

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::{Pubkey, pubkey},
};

use spl_token_2022 as spl_token;

pub struct Check;

impl Check {

    // Whitelist check
    pub fn check_whitelist(payer_account: &AccountInfo) -> ProgramResult {

        const WHITELISTED_USER: [Pubkey;1] = [

            pubkey!("Gkm1jpBw9yv2jrFGNDcHJP2ThL2MsHXGvLT1agbNSyqG"),
        ];

        if !WHITELISTED_USER.contains(payer_account.key) || !payer_account.is_signer {

            return Err(ProgramError::IncorrectAuthority);
        }

        else {

            Ok(())
        }
    }
    
    // system program id check
    pub fn check_system_program(system_program_account: &AccountInfo) -> ProgramResult {

        if !solana_program::system_program::check_id(&system_program_account.key) {

            return Err(ProgramError::InvalidAccountData);
        }

        else {

            Ok(())
        }
    }

    // pda owner check
    pub fn check_pda_owner(pda_import: &AccountInfo, program_id: &Pubkey) -> ProgramResult {

        if pda_import.owner != program_id {

            return Err(ProgramError::IncorrectProgramId);
        }

        else {

            Ok(())
        }
    }

    // pda valid check
    pub fn check_pda_valid(pda_import: &AccountInfo, payer_account: &AccountInfo, program_id: &Pubkey) -> ProgramResult {

        let seeds = &[

            b"create_pda",
            payer_account.key.as_ref(),
        ];

        let (pda_caculate, _) = Pubkey::find_program_address(seeds, program_id);

        if &pda_caculate != pda_import.key {

            return Err(ProgramError::InvalidAccountData);
        }

        else {

            Ok(())
        }
    }

    pub fn check_token_manager(pda_import: &AccountInfo, program_id: &Pubkey) -> ProgramResult {

        Self::_check_token_manager_valid(pda_import,program_id)?;
        Self::_check_token_manager_owner(pda_import,program_id)?;

        Ok(())
    }

    fn _check_token_manager_valid(pda_import: &AccountInfo, program_id: &Pubkey) -> ProgramResult {

        let seeds: &[&[u8]] = &[

            b"token_manager",
        ];
    
        let (pda_caculate, _) = Pubkey::find_program_address(seeds, program_id);
    
        if &pda_caculate != pda_import.key {
    
            return Err(ProgramError::InvalidAccountData);
        }
    
        else {
    
            Ok(())
        }
    }

    fn _check_token_manager_owner(pda_import: &AccountInfo, program_id: &Pubkey) -> ProgramResult {

        if pda_import.owner != program_id {

            return Err(ProgramError::IncorrectProgramId);
        }

        else {

            Ok(())
        }
    }


    // balance check
    pub fn check_balance(account: &AccountInfo, sol_lamports:u64) -> ProgramResult {

        if account.lamports() < sol_lamports {

            return Err(ProgramError::InsufficientFunds);
        }

        else {

            Ok(())
        }
    }

    // is signer check
    pub fn check_is_signer(payer_account: &AccountInfo) -> ProgramResult {

        if !payer_account.is_signer {

            return Err(ProgramError::MissingRequiredSignature);
        }

        else {

            Ok(())
        }
    }

    // check is instruction data empty ?
    pub fn check_instr(instruction_data: &[u8]) -> ProgramResult {

        if instruction_data.is_empty() {

            return Err(ProgramError::InvalidInstructionData);
        }

        else {
            
            Ok(())
        }
    }

    // check token program account
    pub fn check_token_program(token_program_account: &AccountInfo) -> ProgramResult {

        if token_program_account.key != &spl_token::ID {

            return Err(ProgramError::IncorrectProgramId);
        }

        else {

            Ok(())
        }
    }


    // pda vault check
    pub fn check_pda_valut(pda_vault: &AccountInfo,program_id: &Pubkey) -> ProgramResult {

        let seeds: &[&[u8]] = &[

            b"create_pda",
        ];

        let (pda_caculate, _) = Pubkey::find_program_address(seeds, program_id);

        if &pda_caculate != pda_vault.key {

            return Err(ProgramError::InvalidAccountData)
        }

        else {

            Ok(())
        }

    }

}