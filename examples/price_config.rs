use ruint::aliases::U256;

// Constants from the official damm-v2 program
const MIN_SQRT_PRICE: u128 = 4295048016;
const MAX_SQRT_PRICE: u128 = 79226673521066979257578248091;
const RESOLUTION: u8 = 64;

#[derive(PartialEq, Clone, Copy)]
pub enum Rounding {
    Up,
    Down,
}

pub struct PriceConfig {
    // Price of Token A (e.g., SOL) in USD
    token_a_price_usd: f64,
    // Price of Token B (e.g., your SPL token) in USD
    token_b_price_usd: f64,
    // The USD value of the liquidity you want to provide for EACH token.
    usd_value_to_provide: f64,
    // Decimals of Token A
    token_a_decimal: u8,
    // Decimals of Token B
    token_b_decimal: u8,
}

impl PriceConfig {
    pub fn new(
        token_a_price_usd: f64,
        token_b_price_usd: f64,
        usd_value_to_provide: f64,
        token_a_decimal: u8,
        token_b_decimal: u8,
    ) -> Self {
        PriceConfig {
            token_a_price_usd,
            token_b_price_usd,
            usd_value_to_provide,
            token_a_decimal,
            token_b_decimal,
        }
    }

    pub fn get_result(self) -> (u128, u128) {
        let sqrt_price = self._calculate_sqrt_price_x64();
        
        let amount_a_lamports = (self.usd_value_to_provide / self.token_a_price_usd) * 10f64.powi(self.token_a_decimal as i32);
        let amount_b_lamports = (self.usd_value_to_provide / self.token_b_price_usd) * 10f64.powi(self.token_b_decimal as i32);

        let liquidity = self._calculate_liquidity_from_token_amounts(amount_a_lamports, amount_b_lamports);
        
        (sqrt_price, liquidity)
    }

    fn _calculate_sqrt_price_x64(&self) -> u128 {
        let price_ratio = (self.token_b_price_usd / self.token_a_price_usd)
            * (10f64.powi(self.token_a_decimal as i32) / 10f64.powi(self.token_b_decimal as i32));

        let sqrt_price_float = price_ratio.sqrt();
        let sqrt_price_u128 = (sqrt_price_float * 2f64.powi(RESOLUTION as i32)) as u128;

        assert!(sqrt_price_u128 >= MIN_SQRT_PRICE && sqrt_price_u128 <= MAX_SQRT_PRICE, "Calculated sqrt_price is out of valid range");

        sqrt_price_u128
    }

    /// For a full-range pool, liquidity is L = sqrt(x * y).
    /// The on-chain program treats L as a Q64.64 fixed-point number,
    /// so we must scale the result by 2^64.
    fn _calculate_liquidity_from_token_amounts(&self, amount_a: f64, amount_b: f64) -> u128 {
        let product = U256::from(amount_a as u128)
            .checked_mul(U256::from(amount_b as u128))
            .expect("mul overflow");
        
        let sqrt_product = sqrt_u256(product);
        
        // Scale the integer result to Q64.64 format
        let liquidity_q64 = sqrt_product.checked_shl(RESOLUTION as usize).expect("shl overflow");

        liquidity_q64.try_into().expect("liquidity does not fit in u128")
    }
}

/// Calculates the integer square root of a U256 number.
fn sqrt_u256(n: U256) -> U256 {
    if n == U256::ZERO { return U256::ZERO; }
    let mut x = U256::from(1) << ((n.bit_len() + 1) / 2);
    let mut y = (x + n / x) >> 1;
    while y < x {
        x = y;
        y = (x + n / x) >> 1;
    }
    x
}

// --- Check logic as same as the official code ---

pub fn get_initialize_amounts(
    sqrt_min_price: u128,
    sqrt_max_price: u128,
    sqrt_price: u128,
    liquidity: u128,
) -> (u64, u64) {
    // BASE TOKEN
    let amount_a = get_delta_amount_a_unsigned(sqrt_price, sqrt_max_price, liquidity, Rounding::Up);
    // QUOTE TOKEN
    let amount_b = get_delta_amount_b_unsigned(sqrt_min_price, sqrt_price, liquidity, Rounding::Up);
    (amount_a, amount_b)
}


pub fn mul_div_u256(x: U256, y: U256, denominator: U256, rounding: Rounding) -> Option<U256> {
    if denominator == U256::ZERO {
        return None;
    }

    let prod = x.checked_mul(y)?;

    let result = match rounding {
        Rounding::Up => prod.div_ceil(denominator),
        Rounding::Down => {
            let (quotient, _) = prod.div_rem(denominator);
            quotient
        }
    };
    
    Some(result)
}


pub fn get_delta_amount_a_unsigned_unchecked(
    lower_sqrt_price: u128,
    upper_sqrt_price: u128,
    liquidity: u128,
    round: Rounding,
) -> U256 {
    let numerator_1 = U256::from(liquidity);
    let numerator_2 = U256::from(upper_sqrt_price - lower_sqrt_price);

    let denominator = U256::from(lower_sqrt_price).checked_mul(U256::from(upper_sqrt_price)).unwrap();

    assert!(denominator > U256::ZERO);
    let result = mul_div_u256(numerator_1, numerator_2, denominator, round)
        .expect("math overflow");
    result
}


pub fn get_delta_amount_a_unsigned(
    lower_sqrt_price: u128,
    upper_sqrt_price: u128,
    liquidity: u128,
    round: Rounding,
) -> u64 {
    let result = get_delta_amount_a_unsigned_unchecked(
        lower_sqrt_price,
        upper_sqrt_price,
        liquidity,
        round,
    );
    assert!(result <= U256::from(u64::MAX), "math overflow");
    result.try_into().expect("type cast failed")
}


pub fn get_delta_amount_b_unsigned_unchecked(
    lower_sqrt_price: u128,
    upper_sqrt_price: u128,
    liquidity: u128,
    round: Rounding,
) -> U256 {
    let liquidity = U256::from(liquidity);
    let delta_sqrt_price = U256::from(upper_sqrt_price - lower_sqrt_price);
    let prod = liquidity.checked_mul(delta_sqrt_price).unwrap();

    match round {
        Rounding::Up => {
            let denominator = U256::from(1).checked_shl((RESOLUTION as usize) * 2).unwrap();
            prod.div_ceil(denominator)
        }
        Rounding::Down => {
            let (result, _) = prod.overflowing_shr((RESOLUTION as usize) * 2);
            result
        }
    }
}


pub fn get_delta_amount_b_unsigned(
    lower_sqrt_price: u128,
    upper_sqrt_price: u128,
    liquidity: u128,
    round: Rounding,
) -> u64 {
    let result = get_delta_amount_b_unsigned_unchecked(
        lower_sqrt_price,
        upper_sqrt_price,
        liquidity,
        round,
    );
    assert!(result <= U256::from(u64::MAX), "math overflow");
    result.try_into().expect("type cast failed")
}


pub fn verify_with_onchain_logic(sqrt_price: u128, liquidity: u128) -> (u64, u64) {
    let (onchain_amount_a, onchain_amount_b) = get_initialize_amounts(MIN_SQRT_PRICE, MAX_SQRT_PRICE, sqrt_price, liquidity);
    (onchain_amount_a, onchain_amount_b)
}

fn main() {
    // --- Step 1: Client-side Calculation ---
    let sol_price_usd = 180.0;
    let sol_decimal: u8 = 9;
    let spl_price_usd = 0.001;
    let spl_decimal: u8 = 6;
    let usd_value_to_provide = 2.0;

    let (price, liquidity) =
        PriceConfig::new(spl_price_usd, sol_price_usd, usd_value_to_provide, spl_decimal, sol_decimal).get_result();

    println!("--- Client Calculation Results ---");
    println!("sqrt_price (to be sent to chain) = {}", price);
    println!("liquidity (to be sent to chain)  = {}", liquidity);

    // --- Step 2: On-chain Logic Verification ---
    let (onchain_amount_a, onchain_amount_b) = verify_with_onchain_logic(price, liquidity);
    
    println!("\n--- On-chain Calculation Verification ---");
    println!("Based on the client's results, the on-chain program will require:");
    println!("SPL (Token A) lamports: {}", onchain_amount_a);  
    println!("SOL (Token B) lamports: {}", onchain_amount_b); 
    
    // --- Step 3: Compare with Original Intent ---
    let expected_spl_lamports = (usd_value_to_provide / spl_price_usd) * 10f64.powi(spl_decimal as i32);
    let expected_sol_lamports = (usd_value_to_provide / sol_price_usd) * 10f64.powi(sol_decimal as i32);

    println!("\n--- Comparison with Original Intent ---");
    println!("Expected SPL lamports from input: {}", expected_spl_lamports as u64);
    println!("Expected SOL lamports from input: {}", expected_sol_lamports as u64);
}