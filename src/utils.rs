use ethers::types::H160;

pub fn to_string_address(address: &H160) -> String {
    address
        .as_fixed_bytes()
        .iter()
        .fold("0x".to_owned(), |acc, byte| acc + &format!("{:02x}", byte))
}

pub fn to_string_hash(hash: &[u8; 32]) -> String {
    hash.iter()
        .fold("".to_owned(), |acc, byte| acc + &format!("{:02x}", byte))
}
