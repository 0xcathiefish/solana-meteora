use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;


fn main() {


    let a = 1;

    match a {

        1 => {

            let pubkey = Pubkey::from_str("C11DxNAH4NBGNHGzTCq9ZUcJrVJ9dEG5CLwSmis3Y6HJ").unwrap();
            println!("Bytes: {:?}", pubkey.to_bytes());

        },



        2 => {

            let program_id = Pubkey::from_str("H8D2R67fk9ZhqUFMbYY8Saxc8L7yo8cz3un4oWPVS2sM").unwrap();

            let seeds: &[&[u8]] = &[

                b"token_vault"
            ];

            let (pda_caculate, _) = Pubkey::find_program_address(seeds, &program_id);

            println!("Bytes: {:?}", pda_caculate.to_bytes());
            
        },

        _ => {
            
        }


    }

}

