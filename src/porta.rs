//! An implementation of the Porta cipher

/// Represents the mode of operation: encryption or decryption.
#[derive(Clone, Copy)]
enum Mode {
    Encrypt,
    Decrypt,
}

/// Encrypts the provided plaintext using the Porta cipher and the provided key.
///
/// # Arguments
///
/// * `plaintext` - The text to be encrypted.
/// * `key` - The key to use for encryption.
///
/// # Returns
///
/// A string representing the encrypted message (ciphertext).
pub fn encrypt(plaintext: &str, key: &str) -> String {
    encipher(plaintext, key, Mode::Encrypt)
}

/// Decrypts the provided ciphertext using the Porta cipher and the provided key.
///
/// # Arguments
///
/// * `ciphertext` - The text to be decrypted.
/// * `key` - The key to use for decryption.
///
/// # Returns
///
/// A string representing the decrypted message (plaintext).
pub fn decrypt(ciphertext: &str, key: &str) -> String {
    encipher(ciphertext, key, Mode::Decrypt)
}

/// Core logic for the encryption/decryption process.
///
/// This function is responsible for both encryption and decryption, depending on the mode provided.
fn encipher(text: &str, key: &str, mode: Mode) -> String {
    let repeated_key = prepare_key(text, key);
    text.chars()
        .enumerate()
        .filter(|&(_, ch)| ch.is_ascii_alphabetic())
        .map(|(idx, ch)| {
            let text_idx = char_to_index(ch);
            let key_idx = char_to_index(repeated_key.chars().nth(idx).unwrap());

            match mode {
                Mode::Encrypt => index_to_char((text_idx + key_idx) % 13, true),
                Mode::Decrypt => {
                    if text_idx < 13 {
                        index_to_char((13 + text_idx - key_idx) % 13, false)
                    } else {
                        index_to_char((26 + text_idx - key_idx) % 13, true)
                    }
                }
            }
        })
        .collect()
}

/// Prepares the key by repeating it to match the length of the text.
///
/// Returns a string of repeated keys.
fn prepare_key(text: &str, key: &str) -> String {
    key.to_uppercase()
        .chars()
        .cycle()
        .take(text.len())
        .collect()
}

/// Converts a character to its corresponding index in the English alphabet.
fn char_to_index(ch: char) -> usize {
    (ch.to_ascii_uppercase() as usize) - ('A' as usize)
}

/// Converts an index in the English alphabet to its corresponding character.
fn index_to_char(index: usize, is_second_half: bool) -> char {
    let adjusted_index = if is_second_half { index + 13 } else { index };
    char::from_u32((adjusted_index + 'A' as usize) as u32).unwrap()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_porta_encrypt() {
//         let plaintext = "HELLO";
//         let key = "KEY";
//         let encrypted = encrypt(plaintext, key);
//         assert_eq!(encrypted, "RYXXO");
//     }

//     #[test]
//     fn test_porta_decrypt() {
//         let ciphertext = "RYXXO";
//         let key = "KEY";
//         let decrypted = decrypt(ciphertext, key);
//         assert_eq!(decrypted, "HELLO");
//     }

//     #[test]
//     fn test_porta_non_alphabetic_chars() {
//         let plaintext = "HELLO123!";
//         let key = "KEY";
//         let encrypted = encrypt(plaintext, key);
//         assert_eq!(encrypted, "RYXXO");

//         let decrypted = decrypt("RYXXO123!", key);
//         assert_eq!(decrypted, "HELLO");
//     }
// }
