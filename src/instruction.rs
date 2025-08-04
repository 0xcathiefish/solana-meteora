use borsh::{BorshSerialize,BorshDeserialize};

// Meteora instruction

#[derive(BorshSerialize,BorshDeserialize)]
pub enum MeteoraInstruction {

    Swap
}

impl MeteoraInstruction {

    // pack
    pub fn pack(self) -> Vec<u8> {

        self.try_to_vec().expect("Failed to serilize")
    }

    // unpack
    pub fn unpack(instruction_data: &[u8]) -> Result<Self,borsh::maybestd::io::Error> {

        Self::try_from_slice(instruction_data)
    }

}