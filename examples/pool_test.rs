#![allow(unexpected_cfgs)]
#![allow(deprecated)] 

mod constants;
use constants::{seeds,DEFAULT_QUOTE_MINTS};

use anyhow::Result;
use solana_client::rpc_client::RpcClient;

use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    transaction::Transaction,
    compute_budget::ComputeBudgetInstruction,
    pubkey::{Pubkey},
    instruction::{Instruction,AccountMeta},
    system_program
};

use spl_associated_token_account::get_associated_token_address_with_program_id;


use std::{fs, time::Duration};
use log::{info, error};
use dotenvy::dotenv;
use serde_json;

// --- Local Program Dependencies ---
use meteora::instruction::{
    InitializePoolParameters, MeteoraInstruction,
};


fn main() -> Result<()>{

    dotenv().ok();
    env_logger::init();

    let TOKEN_MINT_A: Pubkey = "So11111111111111111111111111111111111111112".parse()?;
    let TOKEN_MINT_B: Pubkey = "HhzuvREDjXYsRoWHauVT1hyZaXRnfxWVwqWFFx1HDLXF".parse()?;

    let CONFIG: Pubkey = "C11DxNAH4NBGNHGzTCq9ZUcJrVJ9dEG5CLwSmis3Y6HJ".parse()?;

    let METEORA_PROGRAM_ID = "cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG".parse()?;


    let (min_mint,max_mint) = pool_get_mints_order(TOKEN_MINT_A,TOKEN_MINT_B);

    let (pool,_) = Pubkey::find_program_address(&[seeds::POOL_PREFIX,CONFIG.as_ref(),max_mint.as_ref(),min_mint.as_ref()], &METEORA_PROGRAM_ID);

    info!("pool pubkey : {:?}",pool);

    Ok(())

}

pub fn pool_get_mints_order(mint_a: Pubkey, mint_b: Pubkey) -> (Pubkey, Pubkey) {

    if mint_a.to_bytes() < mint_b.to_bytes() {

        (mint_a, mint_b)
    }

    else {

        (mint_b, mint_a)
    }
}