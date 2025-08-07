pub struct PriceConfig {

    // 1 spl = ? sol
    sol_per_spl: f64,

    spl_amount_to_provide: f64,

    spl_decimal: u8,
}

impl PriceConfig {

    pub fn new(sol_per_spl: f64, spl_amount_to_provide: f64, spl_decimal: u8) -> Self {

        PriceConfig {

            sol_per_spl,
            spl_amount_to_provide,
            spl_decimal,
        }
    }

    pub fn get_result(self) -> (u128,u128) {

        let sqrt_price = Self::_calculate_sqrt_price_x64(self.sol_per_spl,self.spl_decimal);
        let liquidity = Self::_calculate_liquidity_from_spl_amount(self.spl_amount_to_provide, self.spl_decimal, sqrt_price);

        (sqrt_price,liquidity)
    }


    fn _calculate_sqrt_price_x64(sol_per_spl: f64, spl_decimal: u8) -> u128 {

        let price_p_atomic = sol_per_spl * 10f64.powi(9 as i32 - spl_decimal as i32);
    
        let sqrt_p = price_p_atomic.sqrt();
    
        let q64 = 2.0_f64.powi(64);
    
        let sqrt_price_x64 = sqrt_p * q64;
    
        // Check is valid ?
        if sqrt_price_x64.is_nan() || sqrt_price_x64.is_infinite() {
            panic!("Invalid input");
        }
    
        sqrt_price_x64 as u128
    }


    fn _calculate_liquidity_from_spl_amount(spl_amount: f64, spl_decimal: u8, sqrt_price_x64: u128) -> u128 {

        let spl_amount_atomic = spl_amount * 10f64.powi(spl_decimal as i32);

        let q64 = 2.0_f64.powi(64);
        let liquidity = spl_amount_atomic * (sqrt_price_x64 as f64) / q64;

        // Check
        if liquidity.is_nan() || liquidity.is_infinite() {
            panic!("Invalid input");
        }

        liquidity as u128
    }
}

fn main() {

    // 0.0009 $ = 1 spl token
    let sol_per_spl = 0.000005;

    let spl_amount_to_provide= 13_000_000_000.0;

    let spl_decimal: u8 = 6;

    let (price,liquidity) = PriceConfig::new(sol_per_spl, spl_amount_to_provide, spl_decimal).get_result();

    println!("price = {}, lioquidty = {}",price,liquidity);
}