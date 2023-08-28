//! An implementation of the One-Time Pad (OTP) cipher.

use rand::rngs::OsRng;
use rand::RngCore;

pub fn generate_key(len: usize) -> Vec<u8> {
    // Create a vec of truly random bits of the given length.
    let mut key = Vec::with_capacity(len);
    for _ in 0..len {
        key.push(generate_random_bit())
    }
    key
}

pub fn encrypt(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    if plaintext.len() != key.len() {
        panic!("Key and plaintext lengths do not match!");
    }

    let mut ciphertext: Vec<u8> = vec![];

    for i in 0..plaintext.len() {
        let encrypted_bit = plaintext[i] ^ key[i];
        ciphertext.push(encrypted_bit);
    }

    ciphertext
}

fn generate_random_bit() -> u8 {
    let mut key: Vec<u8> = vec![];
    OsRng.fill_bytes(&mut key);
    let random_u64 = OsRng.next_u64();
    (random_u64 & 1) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_key() {
        let key = generate_key(32);
        println!("Key: {:?}", &key);
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_encrypt() {
        let plaintext = "Hello";
        let key = generate_key(plaintext.len());
        let ciphertext = encrypt(plaintext.as_bytes(), &key);

        assert_eq!(plaintext.len(), key.len());
        assert_eq!(plaintext.len(), ciphertext.len());
        assert_eq!(key.len(), ciphertext.len());
    }
}
