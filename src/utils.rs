use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct SolanaError {
    error_msg: String,
    error_name: String,
    #[allow(unused)]
    error_code_number: u32,
    #[allow(unused)]
    error_origin: Option<String>,
    #[allow(unused)]
    compared_values: Option<String>
}

impl Display for SolanaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {} - {}", self.error_name, self.error_msg)
    }
}

pub fn err(msg: &str) -> SolanaError {
    SolanaError {
        error_msg: msg.to_string(),
        error_name: "Exception".to_string(),
        error_code_number: 0,
        error_origin: None,
        compared_values: None
    }
}

pub fn vec_to_u128(vec: &Vec<u8>) -> u128 {
    let mut array = [0u8; 16];
    array.copy_from_slice(&vec);
    u128::from_be_bytes(array)
}


pub fn bytes_to_binary(i: &[u8; 32], r: &mut Vec<u8>) {
    for m in i.iter() {
        format!("{:8b}", m).chars().for_each(|b| if b == '1' { r.push(1); } else { r.push(0) } );
    }
}