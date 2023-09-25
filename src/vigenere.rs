//! An implementation of the Vigenère cipher
//!
//! This module provides functionality to encrypt and decrypt messages using
//! the Vigenère cipher, a method of encrypting alphabetic text by using a
//! simple form of polyalphabetic substitution.

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALPHABET_SIZE: usize = ALPHABET.len();

/// Enumeration for distinguishing encryption and decryption operations.
///
/// This enum is utilized in the `char_shift` function to indicate whether a
/// character should be shifted for encryption or decryption.
#[derive(Copy, Clone)]
enum Operation {
    Encrypt,
    Decrypt,
}

/// Encrypts the provided plaintext using the Vigenère cipher and the given key.
///
/// # Arguments
///
/// * `plaintext` - The text to be encrypted.
/// * `key` - The key to use for encryption.
///
/// # Returns
///
/// A new string that contains the encrypted version of the `plaintext`.
pub fn encrypt(plaintext: &str, key: &str) -> String {
    encipher(plaintext, key, Operation::Encrypt)
}

/// Decrypts the provided ciphertext using the Vigenère cipher and the given key.
///
/// # Arguments
///
/// * `ciphertext` - The text to be decrypted.
/// * `key` - The key used for decryption.
///
/// # Returns
///
/// A new string that contains the decrypted version of the `ciphertext`.
pub fn decrypt(ciphertext: &str, key: &str) -> String {
    encipher(ciphertext, key, Operation::Decrypt)
}

/// Prepares a string by converting it to uppercase and filtering out non-ASCII-alphabetic characters.
///
/// This function serves as a helper to ensure input consistency before encryption or decryption.
///
/// # Arguments
///
/// * `input_string` - The string to be prepared.
///
/// # Returns
///
/// A new string that has been converted to uppercase and stripped of non-ASCII-alphabetic characters.
pub fn prepare_string(input_string: &str) -> String {
    input_string
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<String>()
        .to_uppercase()
}

/// Transforms the provided text using the Vigenère cipher
/// and the given key based on the specified operation (encryption or decryption).
///
/// This function serves as a core for both encryption and decryption
/// by abstracting out their shared logic. It enciphers the input text
/// depending on the operation specified, either encrypting or decrypting it.
///
/// # Arguments
///
/// * `text` - The text to be transformed (either plaintext for encryption or ciphertext for decryption).
/// * `key` - The key to use for the transformation.
/// * `op` - The operation to be performed (either `Operation::Encrypt` or `Operation::Decrypt`).
///
/// # Returns
///
/// A new string that contains the transformed version of the `text` based on the specified operation.
fn encipher(data: &str, key: &str, op: Operation) -> String {
    let extended_key = extend_key(key, data.len());

    data.chars()
        .enumerate()
        .map(|(idx, c)| {
            let shift = ALPHABET
                .find(extended_key.chars().nth(idx).unwrap_or_default())
                .unwrap_or_default() as isize;
            shift_char(c, shift, op)
        })
        .collect()
}

/// Extends the key to match the desired length by repeating it as necessary.
///
/// This function generates a new key by cycling the provided key until it
/// reaches the desired length.
///
/// # Arguments
///
/// * `key` - The initial key.
/// * `txt_len` - The desired length of the extended key.
///
/// # Returns
///
/// A new string containing the extended key.
fn extend_key(key: &str, txt_len: usize) -> String {
    key.chars().cycle().take(txt_len).collect()
}

/// Shifts a character in the alphabet by a specified amount.
///
/// Depending on the operation (encryption or decryption), the character is
/// shifted forward or backward in the alphabet.
///
/// # Arguments
///
/// * `c` - The character to be shifted.
/// * `shift` - The number of positions to shift the character.
/// * `op` - The operation (either encrypt or decrypt).
///
/// # Returns
///
/// A character that is the result of the shifting operation.
fn shift_char(c: char, shift: isize, op: Operation) -> char {
    let position = ALPHABET.find(c).unwrap_or_default() as isize;

    let new_position = match op {
        Operation::Encrypt => (position + shift).rem_euclid(ALPHABET_SIZE as isize),
        Operation::Decrypt => (position - shift).rem_euclid(ALPHABET_SIZE as isize),
    };

    ALPHABET.chars().nth(new_position as usize).unwrap_or(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vigenere_encipher() {
        let data = "HELLO";
        let key = "KEY";

        // Test encryption
        let encrypted = encipher(
            &prepare_string(data),
            &prepare_string(key),
            Operation::Encrypt,
        );
        assert_eq!(encrypted, "RIJVS");

        // Test decryption
        let decrypted = encipher(&encrypted, &prepare_string(key), Operation::Decrypt);
        assert_eq!(decrypted, "HELLO");
    }

    #[test]
    fn test_vigenere_encrypt() {
        let plaintext = "HELLO";
        let key = "KEY";
        let ciphertext = encrypt(&prepare_string(plaintext), &prepare_string(key));
        assert_eq!(ciphertext, "RIJVS");
    }

    #[test]
    fn test_vigenere_decrypt() {
        let ciphertext = "RIJVS";
        let key = "KEY";
        let plaintext = decrypt(&ciphertext, &prepare_string(key));
        assert_eq!(plaintext, "HELLO");
    }

    #[test]
    fn test_vigenere_prepare_string() {
        let input = "HeLlo WoRld";
        let output = prepare_string(input);
        assert_eq!(output, "HELLOWORLD");
    }

    #[test]
    fn test_vigenere_extend_key() {
        let key = "KEY";
        let extended = extend_key(key, 5);
        assert_eq!(extended, "KEYKE");
    }
}
