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
}
