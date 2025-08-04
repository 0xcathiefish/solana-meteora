#![allow(unexpected_cfgs)]
#![allow(deprecated)]  // 忽略弃用警告

mod instruction;
use crate::instruction::{MeteoraInstruction};

mod check;
use crate::check::Check;


use solana_program::{
    account_info::{next_account_info, AccountInfo}, 
    entrypoint::{ProgramResult}, 
    entrypoint,
    msg, 
    program::{invoke, invoke_signed}, 
    program_error::ProgramError, 
    program_pack::Pack, 
    pubkey::Pubkey, 
    system_instruction, 
    sysvar::{rent::Rent, Sysvar} 
};

use spl_token_2022::{

    instruction as token_instruction,
};

use spl_associated_token_account::{

    instruction as ata_instruction,
};

use borsh::{BorshSerialize,BorshDeserialize};


entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],

) -> ProgramResult {

    Check::check_instr(instruction_data)?;

    let instruction = MeteoraInstruction::unpack(instruction_data)?;

    match instruction {

        _ => {

            
        }
    }

    Ok(())
}
