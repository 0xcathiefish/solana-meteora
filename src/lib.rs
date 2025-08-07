#![allow(unexpected_cfgs)]
#![allow(deprecated)]

pub mod instruction;
use crate::instruction::{InitializePoolParameters, MeteoraInstruction};

pub mod check;
use crate::check::Check;

use borsh::{BorshDeserialize,BorshSerialize};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MeteoraInstruction::unpack(instruction_data)?;

    match instruction {
        MeteoraInstruction::CpiInitializePool(
            InitializePoolParameters {
                liquidity,
                sqrt_price,
                activation_point,
            }
        ) => {
            msg!("Instruction: CpiInitializePool");
            cpi_initialize_pool(
                program_id,
                accounts,
                liquidity,
                sqrt_price,
                activation_point,
            )?;
        },
    }

    Ok(())
}


/// Calls the Meteora DAMM `initialize_pool` instruction.
///
/// This function acts as a proxy, collecting all the necessary accounts and forwarding them
/// in a CPI to the Meteora DAMM program.
#[allow(clippy::too_many_arguments)]
fn cpi_initialize_pool(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    liquidity: u128,
    sqrt_price: u128,
    activation_point: Option<u64>,
) -> ProgramResult {


    msg!("1");

    let accounts_iter = &mut accounts.iter();

    // The order of accounts must match the client-side order exactly.
    let payer                       = next_account_info(accounts_iter)?;
    let creator                     = next_account_info(accounts_iter)?;
    let position_nft_mint           = next_account_info(accounts_iter)?;
    let position_nft_account        = next_account_info(accounts_iter)?;
    let config                      = next_account_info(accounts_iter)?;
    let pool_authority              = next_account_info(accounts_iter)?;
    let pool                        = next_account_info(accounts_iter)?;
    let position                    = next_account_info(accounts_iter)?;
    let token_a_mint                = next_account_info(accounts_iter)?;
    let token_b_mint                = next_account_info(accounts_iter)?;
    let token_a_vault               = next_account_info(accounts_iter)?;
    let token_b_vault               = next_account_info(accounts_iter)?;
    let payer_token_a               = next_account_info(accounts_iter)?;
    let payer_token_b               = next_account_info(accounts_iter)?;
    let token_a_program             = next_account_info(accounts_iter)?;
    let token_b_program             = next_account_info(accounts_iter)?;
    let token_2022_program          = next_account_info(accounts_iter)?;
    let system_program              = next_account_info(accounts_iter)?;
    let meteora_program             = next_account_info(accounts_iter)?;
    let event_authority             = next_account_info(accounts_iter)?;


    // --- Validation Checks ---
    Check::check_is_signer(payer)?;
    Check::check_is_signer(position_nft_mint)?;
    Check::check_system_program(system_program)?;

    // --- Construct CPI Instruction Data ---
    // This struct must match the `InitializePoolParameters` expected by Meteora.
    #[derive(BorshSerialize,BorshDeserialize)]
    struct InitializePoolParametersForCpi {
        liquidity: u128,
        sqrt_price: u128,
        activation_point: Option<u64>,
    }

    let params = InitializePoolParametersForCpi {
        liquidity,
        sqrt_price,
        activation_point,
    };

    // The instruction discriminator for `initialize_pool` is `[149, 82, 72, 197, 253, 252, 68, 15]`
    // initialize_pool is [95,180,10,172,84,174,232,40]
    let mut instruction_data_cpi = vec![95,180,10,172,84,174,232,40];
    instruction_data_cpi.extend_from_slice(&params.try_to_vec().unwrap());

    // --- Construct CPI Accounts ---
    // The order must match Meteora's `initialize_pool` instruction.
    let account_metas = vec![
        AccountMeta::new_readonly(*creator.key, false),
        AccountMeta::new(*position_nft_mint.key, true),
        AccountMeta::new(*position_nft_account.key, false),
        AccountMeta::new(*payer.key, true),
        AccountMeta::new_readonly(*config.key, false),
        AccountMeta::new_readonly(*pool_authority.key, false),
        AccountMeta::new(*pool.key, false),
        AccountMeta::new(*position.key, false),
        AccountMeta::new_readonly(*token_a_mint.key, false),
        AccountMeta::new_readonly(*token_b_mint.key, false),
        AccountMeta::new(*token_a_vault.key, false),
        AccountMeta::new(*token_b_vault.key, false),
        AccountMeta::new(*payer_token_a.key, false),
        AccountMeta::new(*payer_token_b.key, false),
        AccountMeta::new_readonly(*token_a_program.key, false),
        AccountMeta::new_readonly(*token_b_program.key, false),
        AccountMeta::new_readonly(*token_2022_program.key, false),
        AccountMeta::new_readonly(*system_program.key, false),
        AccountMeta::new_readonly(*event_authority.key, false),
        AccountMeta::new_readonly(*meteora_program.key, false)
    ];

    // --- Create and Invoke CPI ---
    let cpi_instruction = Instruction {
        program_id: *meteora_program.key,
        accounts: account_metas,
        data: instruction_data_cpi,
    };

    let account_infos = &[
        creator.clone(),
        position_nft_mint.clone(),
        position_nft_account.clone(),
        payer.clone(),
        config.clone(),
        pool_authority.clone(),
        pool.clone(),
        position.clone(),
        token_a_mint.clone(),
        token_b_mint.clone(),
        token_a_vault.clone(),
        token_b_vault.clone(),
        payer_token_a.clone(),
        payer_token_b.clone(),
        token_a_program.clone(),
        token_b_program.clone(),
        token_2022_program.clone(),
        system_program.clone(),
        event_authority.clone(),
        meteora_program.clone(), // The program being called must be in account_infos
    ];

    msg!("Invoking Meteora DAMM program to initialize pool...");

    invoke(
        &cpi_instruction, 
        account_infos
    )?;
    
    msg!("Pool initialized successfully via CPI");

    Ok(())
}