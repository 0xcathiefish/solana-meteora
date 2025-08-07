#![allow(unexpected_cfgs)]

use solana_sdk::{pubkey::Pubkey};

/// refer raydium clmm
pub const MIN_SQRT_PRICE: u128 = 4295048016;
/// refer raydium clmm
pub const MAX_SQRT_PRICE: u128 = 79226673521066979257578248091;

pub const LIQUIDITY_SCALE: u8 = 128;

pub const REWARD_RATE_SCALE: u8 = 64;

pub const TOTAL_REWARD_SCALE: u8 = 192;

pub const ONE_Q64: u128 = 1u128 << 64;

pub const BIN_STEP_BPS_DEFAULT: u16 = 1;

//  bin_step << 64 / BASIS_POINT_MAX
pub const BIN_STEP_BPS_U128_DEFAULT: u128 = 1844674407370955;


pub const BASIS_POINT_MAX: u64 = 10_000;

pub const U24_MAX: u32 = 0xffffff;

// Number of bits to scale. This will decide the position of the radix point.

// Number of rewards supported by pool
pub const NUM_REWARDS: usize = 2;
pub const REWARD_INDEX_0: usize = 0;
pub const REWARD_INDEX_1: usize = 1;

// Minimum reward duration
pub const MIN_REWARD_DURATION: u64 = 24 * 60 * 60; // 1 day

pub const MAX_REWARD_DURATION: u64 = 31536000; // 1 year = 365 * 24 * 3600

pub mod activation {
    #[cfg(not(feature = "local"))]
    pub const SLOT_BUFFER: u64 = 9000; // 1 slot = 400 mls => 1 hour
    #[cfg(feature = "local")]
    pub const SLOT_BUFFER: u64 = 5;

    #[cfg(not(feature = "local"))]
    pub const TIME_BUFFER: u64 = 3600; // 1 hour
    #[cfg(feature = "local")]
    pub const TIME_BUFFER: u64 = 5; // 5 secs

    #[cfg(not(feature = "local"))]
    pub const MAX_ACTIVATION_SLOT_DURATION: u64 = SLOT_BUFFER * 24 * 31; // 31 days
    #[cfg(feature = "local")]
    pub const MAX_ACTIVATION_SLOT_DURATION: u64 = 30;

    #[cfg(not(feature = "local"))]
    pub const MAX_ACTIVATION_TIME_DURATION: u64 = TIME_BUFFER * 24 * 31; // 31 days
    #[cfg(feature = "local")]
    pub const MAX_ACTIVATION_TIME_DURATION: u64 = 30;

    pub const MAX_VESTING_SLOT_DURATION: u64 = SLOT_BUFFER * 24 * 365 * 10; // 10 years
    pub const MAX_VESTING_TIME_DURATION: u64 = TIME_BUFFER * 24 * 365 * 10; // 10 years

    pub const FIVE_MINUTES_SLOT_BUFFER: u64 = SLOT_BUFFER / 12; // 5 minutes

    pub const FIVE_MINUTES_TIME_BUFFER: u64 = TIME_BUFFER / 12; // 5 minutes

    pub const MAX_FEE_CURVE_TIME_DURATION: u64 = 3600 * 24; // 1 day
    pub const MAX_FEE_CURVE_SLOT_DURATION: u64 = 9000 * 24; // 1 day

    pub const MAX_HIGH_TAX_TIME_DURATION: u64 = TIME_BUFFER / 6; // 10 minutes
    pub const MAX_HIGH_TAX_SLOT_DURATION: u64 = SLOT_BUFFER / 6; // 10 minutes
}

/// Store constants related to fees
pub mod fee {

    /// Default fee denominator. DO NOT simply update it as it will break logic that depends on it as default value.
    pub const FEE_DENOMINATOR: u64 = 1_000_000_000;

    /// Max fee BPS
    pub const MAX_FEE_BPS: u64 = 5000; // 50%
    pub const MAX_FEE_NUMERATOR: u64 = 500_000_000; // 50%

    /// Max basis point. 100% in pct
    pub const MAX_BASIS_POINT: u64 = 10000;

    pub const MIN_FEE_BPS: u64 = 1; // 0.01%
    pub const MIN_FEE_NUMERATOR: u64 = 100_000;


    pub const PROTOCOL_FEE_PERCENT: u8 = 20; // 20%

    pub const HOST_FEE_PERCENT: u8 = 20; // 20% of protocol fee

    pub const PARTNER_FEE_PERCENT: u8 = 0; // percentage of partner fee

}

pub mod seeds {
    pub const CONFIG_PREFIX: &[u8] = b"config";
    pub const CUSTOMIZABLE_POOL_PREFIX: &[u8] = b"cpool";
    pub const POOL_PREFIX: &[u8] = b"pool";
    pub const TOKEN_VAULT_PREFIX: &[u8] = b"token_vault";
    pub const POOL_AUTHORITY_PREFIX: &[u8] = b"pool_authority";
    pub const POSITION_PREFIX: &[u8] = b"position";
    pub const POSITION_NFT_ACCOUNT_PREFIX: &[u8] = b"position_nft_account";
    pub const TOKEN_BADGE_PREFIX: &[u8] = b"token_badge";
    pub const REWARD_VAULT_PREFIX: &[u8] = b"reward_vault";
    pub const CLAIM_FEE_OPERATOR_PREFIX: &[u8] = b"cf_operator";
    pub const EVENT_AUTHORITY: &[u8] = b"__event_authority";
}


// Supported quote mints
const SOL: Pubkey = Pubkey::new_from_array([6, 155, 136, 87, 254, 171, 129, 132, 251, 104, 127, 99, 70, 24, 192, 53, 218, 196, 57, 220, 26, 235, 59, 85, 152, 160, 240, 0, 0, 0, 0, 1]);
const USDC: Pubkey = Pubkey::new_from_array([198, 250, 122, 243, 190, 219, 173, 58, 61, 101, 243, 106, 171, 201, 116, 49, 177, 187, 228, 194, 210, 246, 224, 228, 124, 166, 2, 3, 69, 47, 93, 97]);
pub const DEFAULT_QUOTE_MINTS: [Pubkey; 2] = [SOL, USDC];

fn main() {}