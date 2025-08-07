use borsh::{BorshDeserialize, BorshSerialize};

/// The constant-product AMM instruction data.
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub enum MeteoraInstruction {
    
    // Meteora Damm V2 -> initialize_pool
    CpiInitializePool(InitializePoolParameters),
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct InitializePoolParameters {
    pub liquidity: u128,
    pub sqrt_price: u128,
    pub activation_point: Option<u64>,
}

impl MeteoraInstruction {
    
    // pack
    pub fn pack(self) -> Vec<u8> {
        self.try_to_vec().expect("Failed to serilize")
    }

    // unpack
    pub fn unpack(instruction_data: &[u8]) -> Result<Self, borsh::maybestd::io::Error> {
        Self::try_from_slice(instruction_data)
    }
}