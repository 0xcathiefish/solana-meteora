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

// --- Constants ---
const RPC_URL: &str = "https://api.devnet.solana.com";



// devnet meteora program id  cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG
const METEORA_PROGRAM_ID: Pubkey = Pubkey::new_from_array([9, 45, 33, 53, 101, 122, 21, 156, 43, 135, 212, 182, 106, 112, 219, 142, 151, 82, 56, 159, 247, 106, 175, 32, 108, 237, 6, 58, 56, 249, 90, 237]);

// wsol
const TOKEN_MINT_A: Pubkey = DEFAULT_QUOTE_MINTS[0];

// HhzuvREDjXYsRoWHauVT1hyZaXRnfxWVwqWFFx1HDLXF spl-token22
const TOKEN_MINT_B: Pubkey = Pubkey::new_from_array([248, 59, 180, 121, 130, 62, 40, 43, 72, 167, 37, 154, 80, 64, 86, 42, 98, 50, 225, 212, 118, 32, 132, 4, 4, 178, 195, 119, 133, 115, 134, 38]);

// CONFIG C11DxNAH4NBGNHGzTCq9ZUcJrVJ9dEG5CLwSmis3Y6HJ
const CONFIG: Pubkey = Pubkey::new_from_array([163, 112, 207, 9, 111, 69, 176, 4, 63, 43, 120, 87, 144, 5, 113, 224, 14, 223, 88, 68, 124, 75, 2, 106, 243, 32, 47, 6, 98, 37, 10, 221]);

fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    // on-chain program id 9AALRRB5DfN2gNT7QmRKeQdRS5VGvZaoYBqkBQSXaAAb
    let PROGRAM_ID: Pubkey = "3HXXH4ypbLt8MFVw54umbZFWkRwFuuRpVGsXGBvoxaUq".parse()?;

    let client = RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::confirmed());
    info!("Connected to RPC: {}", RPC_URL);

    let wallet_string = fs::read_to_string("/home/xiannvweideta/.config/solana/dev.json")?;
    let keypair_bytes: Vec<u8> = serde_json::from_str(&wallet_string)?;
    let payer = Keypair::try_from(&keypair_bytes[..])?;

    info!("Payer account: {}", payer.pubkey());

    // --- Airdrop if needed ---
    match client.get_balance(&payer.pubkey()) {
        Ok(balance) if balance < 2_000_000_000 => {
            info!("Requesting airdrop...");
            let sig = client.request_airdrop(&payer.pubkey(), 2_000_000_000)?; // 2 SOL
            info!("Airdrop signature: {}", sig);
            loop {
                if let Ok(confirmed) = client.confirm_transaction(&sig) {
                    if confirmed {
                        info!("✅ Airdrop confirmed!");
                        break;
                    }
                }
                std::thread::sleep(Duration::from_secs(1));
            }
        }
        Ok(balance) => info!("Payer balance: {} SOL", balance as f64 / 1_000_000_000.0),
        Err(e) => error!("Failed to get balance: {}", e),
    }


    // account generate
    let position_nft_mint = Keypair::new();
    let position_nft_mint_pub = position_nft_mint.pubkey();
    info!("nft_mint pubkey :{}",position_nft_mint_pub);


    let (position_nft_account_pda,_) = Pubkey::find_program_address(&[seeds::POSITION_NFT_ACCOUNT_PREFIX,position_nft_mint_pub.as_ref()], &METEORA_PROGRAM_ID);

    // The pool authority is a constant address specified in the IDL, not a PDA.
    let pool_authority_pda = "HLnpSz9h2S4hiLQ43rnSD9XkcUThA7B8hQMKmDaiTLcC".parse()?;


    let (min_mint,max_mint) = pool_get_mints_order(TOKEN_MINT_A,TOKEN_MINT_B);

    let (pool,_) = Pubkey::find_program_address(&[seeds::POOL_PREFIX,CONFIG.as_ref(),max_mint.as_ref(),min_mint.as_ref()], &METEORA_PROGRAM_ID);

    info!("pool pubkey : {:?}",pool);

    //return Ok(());

    let (position_pda, _) = Pubkey::find_program_address(&[seeds::POSITION_PREFIX, position_nft_mint_pub.as_ref()], &METEORA_PROGRAM_ID);
    let (token_a_vault_pda, _) = Pubkey::find_program_address(&[seeds::TOKEN_VAULT_PREFIX, TOKEN_MINT_A.as_ref(), pool.as_ref(),], &METEORA_PROGRAM_ID);
    let (token_b_vault_pda, _) = Pubkey::find_program_address(&[seeds::TOKEN_VAULT_PREFIX, TOKEN_MINT_B.as_ref(), pool.as_ref(),], &METEORA_PROGRAM_ID);

    let creator_token_a_ata = "H3KxVJT77tyNzj6QAEkDajWTCy6uYE7Rzhs7MBGRdg8k".parse()?;
    let creator_token_b_ata = "47zz3JfQD7FnM9QSK82VVdytmdDQd4W715zoRiTeNr6e".parse()?;

    let (event_authority_pda,_) = Pubkey::find_program_address(&[seeds::EVENT_AUTHORITY], &METEORA_PROGRAM_ID);

    // --- Build and Send CPI Transaction ---
    let cpi_params = MeteoraInstruction::CpiInitializePool(
        InitializePoolParameters {
            liquidity: 100_000_000,
            sqrt_price: 79228162514,
            activation_point: None,
        },
    );

    let cpi_ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            // User and signer
            // 0. payer tx signer
            AccountMeta::new(payer.pubkey(), true),

            // 1. pool creator = payer
            AccountMeta::new_readonly(payer.pubkey(), false),

            // 2.nft mint = random
            AccountMeta::new(position_nft_mint.pubkey(), true),

            // 3. the ATA account to store the mint token of 2
            AccountMeta::new(position_nft_account_pda, false),

            // 4. pool config account
            AccountMeta::new_readonly(CONFIG, false),

            // 5. pool manager pda
            AccountMeta::new_readonly(pool_authority_pda, false),

            // 6. store all pool status account
            AccountMeta::new(pool, false),

            // 7. account store all open position info
            AccountMeta::new(position_pda, false),

            // 8. token a mint account
            AccountMeta::new_readonly(TOKEN_MINT_A, false),

            // 9. token b mint account
            AccountMeta::new_readonly(TOKEN_MINT_B, false),

            // 10. token a vault pda account
            AccountMeta::new(token_a_vault_pda, false),

            // 11. token b vault pda account
            AccountMeta::new(token_b_vault_pda, false),

            // 12. account which used by user to transfer the token mint a, which is used for provide liquity
            AccountMeta::new(creator_token_a_ata, false),

            // 13. account which used by user to transfer the token mint b, which is usually the spl token
            AccountMeta::new(creator_token_b_ata, false),

            // 14. spl token program for token a
            AccountMeta::new_readonly(spl_token::ID, false),

            // 15. spl token program for token B (must be token_2022 for a token_2022 mint)
            AccountMeta::new_readonly(spl_token_2022::ID, false),

            // 16. token22 program account
            AccountMeta::new_readonly(spl_token_2022::ID, false),

            // 17. system program account
            AccountMeta::new_readonly(system_program::id(), false),

            // 18. meteora program account
            AccountMeta::new_readonly(METEORA_PROGRAM_ID, false),

            // 19. Event authority (must be readonly)
            AccountMeta::new_readonly(event_authority_pda, false),
        ],
        data: cpi_params.pack(),
    };

    let mut cpi_tx = Transaction::new_with_payer(
        &[
            ComputeBudgetInstruction::set_compute_unit_limit(400_000),
            cpi_ix
        ],
        Some(&payer.pubkey()),
    );

    let recent_blockhash = client.get_latest_blockhash()?;
    cpi_tx.sign(&[&payer, &position_nft_mint], recent_blockhash);

    info!("Sending CPI transaction to create Meteora pool...");
    let sig = client.send_and_confirm_transaction(&cpi_tx)?;
    info!("✅ CPI transaction successful!");
    info!("Transaction Signature: {}", sig);
    info!("🔍 View on Explorer: https://solscan.io/tx/{}?cluster=custom&customUrl={}", sig, RPC_URL);
    info!("Meteora Pool Address: {}", pool);

    Ok(())
}

// Return thee correct mint order to config pool
pub fn pool_get_mints_order(mint_a: Pubkey, mint_b: Pubkey) -> (Pubkey, Pubkey) {

    if mint_a.to_bytes() < mint_b.to_bytes() {

        (mint_a, mint_b)
    }

    else {

        (mint_b, mint_a)
    }
}