pub fn hash_block_data(data: &str, prev_hash: &str, nonce: u64) -> String {
    sha256::digest(format!("{}{}{}", data, prev_hash, nonce).as_bytes())
}

pub fn target_zeros_amount(difficulty: f64) -> usize {
    difficulty.log2() as usize
}

pub fn zeros_amount(hash: &str) -> usize {
    let bytes = hex::decode(hash).unwrap();

    let mut leading_zeros = 0;

    for byte in bytes {
        if byte == 0 {
            leading_zeros += 8;
        } else {
            let mut mask = 0b10000000;
            while mask > 0 {
                if byte & mask == 0 {
                    leading_zeros += 1;
                    mask >>= 1;
                } else {
                    break;
                }
            }
            break;
        }
    }

    leading_zeros
}

pub fn validate_hash(hash: &str, difficulty: f64) -> bool {
    let target_zeros = target_zeros_amount(difficulty);
    let leading_zeros = zeros_amount(hash);

    leading_zeros >= target_zeros
}
