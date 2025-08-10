#![allow(unexpected_cfgs)]
#![allow(deprecated)]

pub mod instruction;
use crate::instruction::{MeteoraInstruction};

pub mod check;
use crate::check::Check;

pub mod meteora_v2_pool;
pub use meteora_v2_pool::{
    
    TradeDirection,
    MeteoraDammV2Pool,
    MeteoraDammV2PoolSwapParams,
    InitializePoolParameters
};

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
            InitializePoolParameters,
        ) => {
            msg!("Instruction: CpiInitializePool");
            cpi_initialize_pool(
                program_id,
                accounts,
                InitializePoolParameters,
            )?;
        },

        MeteoraInstruction::CpiSwap(
            MeteoraDammV2PoolSwapParams,
            TradeDirection,
        ) => {
            msg!("Instruction: CpiSwap");
            cpi_swap(
                program_id,
                accounts,
                MeteoraDammV2PoolSwapParams,
                TradeDirection,
            )?;
        }

        _ => {}
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
    InitializePoolParameters: InitializePoolParameters,
) -> ProgramResult {

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

    let params = InitializePoolParameters;

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



#[allow(clippy::too_many_arguments)]
fn cpi_swap(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    MeteoraDammV2PoolSwapParams: MeteoraDammV2PoolSwapParams,
    TradeDirection: TradeDirection,
) -> ProgramResult {

    msg!("Instruction to swap");
    msg!("amount in : {}", MeteoraDammV2PoolSwapParams.amount_in);
    msg!("minimum_amount_out : {}", MeteoraDammV2PoolSwapParams.minimum_amount_out);
    
    let accounts_iter = &mut accounts.iter();

    // The order of accounts must match the client-side order exactly.
    let payer                       = next_account_info(accounts_iter)?;
    let pool_authority              = next_account_info(accounts_iter)?;
    let pool                        = next_account_info(accounts_iter)?;

    let (
        input_token_account,
        output_token_account,

    ) = match TradeDirection {

        TradeDirection::BUY => {

            let input_token_account         = next_account_info(accounts_iter)?;
            let output_token_account        = next_account_info(accounts_iter)?;
            
            (input_token_account,output_token_account)
        },

        TradeDirection::SELL => {

            let input_token_account         = next_account_info(accounts_iter)?;
            let output_token_account        = next_account_info(accounts_iter)?;
            
            (output_token_account,input_token_account)
        }
    };

    
    let token_a_vault               = next_account_info(accounts_iter)?;
    let token_b_vault               = next_account_info(accounts_iter)?;
    let token_a_mint                = next_account_info(accounts_iter)?;
    let token_b_mint                = next_account_info(accounts_iter)?;
    let token_a_program             = next_account_info(accounts_iter)?;
    let token_b_program             = next_account_info(accounts_iter)?;    
    let referral_token_account      = next_account_info(accounts_iter)?;
    let event_authority             = next_account_info(accounts_iter)?;
    let meteora_program             = next_account_info(accounts_iter)?;
    
    

    // --- Validation Checks ---
    Check::check_is_signer(payer)?;


    let params = MeteoraDammV2PoolSwapParams;

    // The instruction discriminator for `swap` 
    // 需要通过测试或IDL确定，这里使用占位符
    let mut instruction_data_cpi = vec![248,198,158,145,225,117,135,200]; 
    instruction_data_cpi.extend_from_slice(&params.try_to_vec().unwrap());

    // --- Construct CPI Accounts ---
    // The order must match Meteora's `swap` instruction.
    let account_metas = vec![
        AccountMeta::new_readonly(*pool_authority.key, false),
        AccountMeta::new(*pool.key, false),
        AccountMeta::new(*input_token_account.key, false),
        AccountMeta::new(*output_token_account.key, false),
        AccountMeta::new(*token_a_vault.key, false),
        AccountMeta::new(*token_b_vault.key, false),
        AccountMeta::new_readonly(*token_a_mint.key, false),
        AccountMeta::new_readonly(*token_b_mint.key, false),
        AccountMeta::new_readonly(*payer.key, true),
        AccountMeta::new_readonly(*token_a_program.key, false),
        AccountMeta::new_readonly(*token_b_program.key, false),
        // referral account 暂时跳过
        AccountMeta::new(*referral_token_account.key, false),
        AccountMeta::new_readonly(*event_authority.key, false),
        AccountMeta::new_readonly(*meteora_program.key, false),
    ];

    // --- Create and Invoke CPI ---
    let cpi_instruction = Instruction {
        program_id: *meteora_program.key,
        accounts: account_metas,
        data: instruction_data_cpi,
    };

    let account_infos = &[
        pool_authority.clone(),
        pool.clone(),
        input_token_account.clone(),
        output_token_account.clone(),
        token_a_vault.clone(),
        token_b_vault.clone(),
        token_a_mint.clone(),
        token_b_mint.clone(),
        payer.clone(),
        token_a_program.clone(),
        token_b_program.clone(),
        referral_token_account.clone(), 
        event_authority.clone(),
        meteora_program.clone(), // The program being called must be in account_infos
    ];

    msg!("Invoking Meteora DAMM program to swap...");

    invoke(
        &cpi_instruction, 
        account_infos
    )?;
    
    msg!("Swap executed successfully via CPI");

    Ok(())
}
