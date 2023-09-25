//! An implementation of the One-Time Pad (OTP) cipher.

use rand::rngs::OsRng;
use rand::RngCore;

/// Generates a key of the specified length using true randomness.
///
/// # Arguments
///
/// * `len` - The desired length of the key, usually derived from
/// the length of the plaintext and/or ciphertext.
pub fn generate_key(len: usize) -> Vec<u8> {
    let mut key = vec![0u8; len];
    OsRng.fill_bytes(&mut key);
    key
}

/// Encrypts the given plaintext using the specified key.
///
/// # Arguments
///
/// * `plaintext` - The data to be encrypted.
/// * `key` - The key to use for encryption.
pub fn encrypt(plaintext: &str, key: &[u8]) -> Vec<u8> {
    encipher(plaintext.as_bytes(), key)
}

/// Decrypts the given ciphertext using the specified key.
///
/// # Arguments
///
/// * `ciphertext` - The data to be decrypted.
/// * `key` - The key to use for decryption.
pub fn decrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    encipher(ciphertext, key)
}

/// Encrypts or decrypts the data using the given key.
/// This generic function can be used for both encryption and decryption since the OTP is symmetrical.
///
/// # Arguments
///
/// * `data` - The plaintext (for encryption) or ciphertext (for decryption).
/// * `key` - The key to use for encryption/decryption.
fn encipher(data: &[u8], key: &[u8]) -> Vec<u8> {
    if data.len() != key.len() {
        panic!("The lengths of the data and the key do not match!");
    }

    data.iter().zip(key.iter()).map(|(a, b)| a ^ b).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_otp_key_generation() {
        let key = generate_key(32);
        println!("Key: {:?}", &key);
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_otp_encryption_and_decryption() {
        let plaintext = "Hello";
        let key = generate_key(plaintext.len());
        let ciphertext = encrypt(plaintext, &key);

        println!(
            "Plaintext: {}, \nKey: {:?}, \nCiphertext: {:?}",
            plaintext, key, ciphertext
        );

        assert_eq!(plaintext.len(), key.len());
        assert_eq!(plaintext.len(), ciphertext.len());
        assert_eq!(key.len(), ciphertext.len());

        let decrypted_plaintext = decrypt(&ciphertext, &key);
        let decrypted_plaintext = unsafe { String::from_utf8_unchecked(decrypted_plaintext) };

        println!("\nDecrypted Plaintext: {}", decrypted_plaintext);

        assert_eq!(decrypted_plaintext.len(), plaintext.len());
        assert_eq!(decrypted_plaintext, plaintext);
    }

    #[test]
    fn test_otp_encipherment() {
        let plaintext = "Hello";
        let key = generate_key(plaintext.len());
        let ciphertext = encipher(plaintext.as_bytes(), &key);

        println!(
            "Plaintext: {}, \nKey: {:?}, \nCiphertext: {:?}",
            plaintext, key, ciphertext
        );

        assert_eq!(plaintext.len(), key.len());
        assert_eq!(plaintext.len(), ciphertext.len());
        assert_eq!(key.len(), ciphertext.len());

        let decrypted_plaintext = encipher(&ciphertext, &key);
        let decrypted_plaintext = unsafe { String::from_utf8_unchecked(decrypted_plaintext) };

        println!("\nDecrypted Plaintext: {}", decrypted_plaintext);

        assert_eq!(decrypted_plaintext.len(), plaintext.len());
        assert_eq!(decrypted_plaintext, plaintext);
    }
}
