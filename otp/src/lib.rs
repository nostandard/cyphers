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
        // Encrypt the plaintext using the key
        let encrypted_bit = plaintext[i] ^ key[i];
        ciphertext.push(encrypted_bit);
    }

    ciphertext
}

pub fn decrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    if ciphertext.len() != key.len() {
        panic!("Key and ciphertext lengths do not match!");
    }

    let mut plaintext: Vec<u8> = vec![];

    for i in 0..ciphertext.len() {
        // Decrypt the ciphertext using the key
        let decrypted_bit = ciphertext[i] ^ key[i];
        plaintext.push(decrypted_bit);
    }

    plaintext
}

// Generic implementation for both encryption and decryption
pub fn encipher(data: &[u8], key: &[u8]) -> Vec<u8> {
    if data.len() != key.len() {
        panic!("The lengths of the data and the key do not match!");
    }

    let mut enciphered_data: Vec<u8> = vec![];

    for i in 0..data.len() {
        // Encrypt/decrypt the data using the key
        let enciphered_bit = data[i] ^ key[i];
        enciphered_data.push(enciphered_bit);
    }

    enciphered_data
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
    fn test_encrypt_decrypt() {
        let plaintext = "Hello";
        let key = generate_key(plaintext.len());
        let ciphertext = encrypt(plaintext.as_bytes(), &key);

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
    fn test_encipher() {
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
