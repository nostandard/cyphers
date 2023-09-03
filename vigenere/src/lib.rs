//! An implementation of the Vigenère cipher
//!
//! This crate provides functionality to encrypt and decrypt messages using
//! the Vigenère cipher, a method of encrypting alphabetic text by using a
//! simple form of polyalphabetic substitution.

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALPHABET_SIZE: usize = ALPHABET.len();

/// Enumeration for distinguishing encryption and decryption operations.
///
/// This enum is utilized in the `char_shift` function to indicate whether a
/// character should be shifted for encryption or decryption.
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
    let extended_key = extend_key(key, plaintext.len());

    plaintext
        .chars()
        .enumerate()
        .map(|(idx, c)| {
            let shift = ALPHABET
                .find(extended_key.chars().nth(idx).unwrap_or_default())
                .unwrap_or_default() as isize;
            shift_char(c, shift, Operation::Encrypt)
        })
        .collect()
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
    let extended_key = extend_key(key, ciphertext.len());

    ciphertext
        .chars()
        .enumerate()
        .map(|(idx, char)| {
            let shift = ALPHABET
                .find(extended_key.chars().nth(idx).unwrap_or_default())
                .unwrap_or_default() as isize;
            shift_char(char, shift, Operation::Decrypt)
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

#[doc(hidden)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vigenere_encryption() {
        let plaintext = "HELLO";
        let key = "KEY";
        let ciphertext = encrypt(&prepare_string(plaintext), &prepare_string(key));
        assert_eq!(ciphertext, "RIJVS");
    }

    #[test]
    fn test_vigenere_decryption() {
        let ciphertext = "RIJVS";
        let key = "KEY";
        let plaintext = decrypt(&ciphertext, &prepare_string(key));
        assert_eq!(plaintext, "HELLO");
    }

    #[test]
    fn test_prepare_string() {
        let input = "HeLlo WoRld";
        let output = prepare_string(input);
        assert_eq!(output, "HELLOWORLD");
    }

    #[test]
    fn test_extend_key() {
        let key = "KEY";
        let extended = extend_key(key, 5);
        assert_eq!(extended, "KEYKE");
    }
}
