use borsh::{BorshDeserialize,BorshSerialize};
use ruint::aliases::U256;

// expected total data bytes
const EXPECT_TOTAL_BYTES: usize = 1112;
const DISCRIMINATOR: usize = 8;

// liquidity bytes range
pub const LIQUIDITY_BEGIN: usize = 352 + DISCRIMINATOR;
pub const LIQUIDITY_END: usize = LIQUIDITY_BEGIN + 16;

// sqrt bytes range
pub const SQRT_PRICE_BEGIN: usize = 448 + DISCRIMINATOR;
pub const SQRT_PRICE_END: usize = SQRT_PRICE_BEGIN + 16;


#[derive(BorshSerialize,BorshDeserialize)]
pub enum  TradeDirection{

    BUY,
    SELL,
}


// initialize_pool
#[derive(BorshSerialize, BorshDeserialize)]
pub struct InitializePoolParameters {
    pub liquidity: u128,
    pub sqrt_price: u128,
    pub activation_point: Option<u64>,
}




#[derive(BorshSerialize,BorshDeserialize)]
pub struct MeteoraDammV2Pool {
   
    total_bytes: usize,
    pub liquidity: u128,
    pub sqrt_price: u128,
}

impl MeteoraDammV2Pool {

    pub fn new(total_bytes: usize, liquidity: u128, sqrt_price: u128) -> Self {

        assert_eq!(total_bytes,EXPECT_TOTAL_BYTES);

        MeteoraDammV2Pool {

            total_bytes,
            liquidity,
            sqrt_price,
        }
    }
}



#[derive(BorshDeserialize,BorshSerialize)]
pub struct MeteoraDammV2PoolSwapParams {

    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

impl MeteoraDammV2PoolSwapParams {
    pub fn new(direction: bool, liquidity: u128, sqrt_price: u128, amount_in: u64, slipage_bps: u64) -> Self {
        
        if liquidity == 0 || sqrt_price == 0 || amount_in == 0 {
            return MeteoraDammV2PoolSwapParams {
                amount_in,
                minimum_amount_out: 0,
            };
        }
        
        // 转换为U256避免溢出
        let liquidity_256 = U256::from(liquidity);
        let sqrt_price_256 = U256::from(sqrt_price);
        let amount_in_256 = U256::from(amount_in);
        
        let amount_out = if direction {
            // A to B: 实现 get_next_sqrt_price_from_amount_a_rounding_up + get_delta_amount_b_unsigned
            
            // Step 1: 计算 next_sqrt_price = sqrt_price * liquidity / (liquidity + amount_in * sqrt_price)
            let product = amount_in_256 * sqrt_price_256;
            let denominator = liquidity_256 + product;
            
            if denominator == U256::ZERO {
                0
            } else {
                let next_sqrt_price = (liquidity_256 * sqrt_price_256) / denominator;
                
                // Step 2: 计算 output = liquidity * (sqrt_price - next_sqrt_price) / 2^128
                let price_diff = sqrt_price_256 - next_sqrt_price;
                let numerator = liquidity_256 * price_diff;
                // 除以 2^128 (RESOLUTION * 2 = 64 * 2 = 128)
                let result: ruint::Uint<256, 4> = numerator >> 128;
                result.try_into().unwrap_or(0u64)
            }
        } else {
            // B to A: 实现 get_next_sqrt_price_from_amount_b_rounding_down + get_delta_amount_a_unsigned
            
            // Step 1: 计算 next_sqrt_price = sqrt_price + (amount_in * 2^128) / liquidity
            let amount_shifted = amount_in_256 << 128; // 乘以 2^128
            let price_increase = amount_shifted / liquidity_256;
            let next_sqrt_price = sqrt_price_256 + price_increase;
            
            // Step 2: 计算 output = liquidity * (next_sqrt_price - sqrt_price) / (sqrt_price * next_sqrt_price)
            let price_diff = next_sqrt_price - sqrt_price_256;
            let numerator = liquidity_256 * price_diff;
            let denominator = sqrt_price_256 * next_sqrt_price;
            
            if denominator == U256::ZERO {
                0
            } else {
                let result: ruint::Uint<256, 4> = numerator / denominator;
                result.try_into().unwrap_or(0u64)
            }
        };
        
        // 应用滑点保护
        let minimum_amount_out = amount_out.saturating_mul(10_000 - slipage_bps) / 10_000;
        
        MeteoraDammV2PoolSwapParams {
            amount_in,
            minimum_amount_out,
        }
    }
}