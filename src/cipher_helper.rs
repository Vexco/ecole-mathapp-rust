pub struct CipherHelper;

impl CipherHelper {
    pub fn cipher(key: (i64, i64), message: &[i64]) -> Vec<i64> {
        message.iter().map(|&m| ((key.0 * m + key.1).rem_euclid(26)) + 65 ).collect()
    }

    pub fn decipher(pub_key: (i64, i64), cipher_message: Vec<i64>) -> Vec<i64> {
        cipher_message
            .iter()
            .map(|&m| ((pub_key.0 * (m - pub_key.1)).rem_euclid(26) + 65))
            .collect()
    }
}