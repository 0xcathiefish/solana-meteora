use borsh::{BorshDeserialize, BorshSerialize};

use crate::meteora_v2_pool::{
    TradeDirection,
    InitializePoolParameters,
    MeteoraDammV2PoolSwapParams
};

/// The constant-product AMM instruction data.
#[derive(BorshSerialize, BorshDeserialize)]
pub enum MeteoraInstruction {
    
    // Meteora Damm V2 -> initialize_pool
    CpiInitializePool(InitializePoolParameters),
    CpiSwap(MeteoraDammV2PoolSwapParams,TradeDirection),
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