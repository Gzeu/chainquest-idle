pub fn encrypt(data: &[u8], key: &[u8; 16]) -> Vec<u8> {
    // Placeholder XOR-based mock (replace with proper crypto crate in prod)
    data.iter().enumerate().map(|(i, b)| b ^ key[i % 16]).collect()
}

pub fn decrypt(data: &[u8], key: &[u8; 16]) -> Vec<u8> {
    encrypt(data, key)
}
