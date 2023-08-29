//! An implementation of the Caesar cipher.

/// Modes of operation for the Caesar cipher.
///
/// The two modes of operation are `Encrypt` for encryption and `Decrypt` for decryption.
enum Mode {
    Encrypt,
    Decrypt,
}

/// Encrypts a given plaintext string using the Caesar cipher.
///
/// # Arguments
///
/// * `plaintext` - The input string to be encrypted.
/// * `key` - The shift value (number of positions each letter is moved in the alphabet).
///
/// # Returns
///
/// The encrypted version of the input string.
pub fn encrypt(plaintext: &str, key: i32) -> String {
    encipher(Mode::Encrypt, plaintext, key)
}

/// Decrypts a given ciphertext string using the Caesar cipher.
///
/// # Arguments
///
/// * `ciphertext` - The encrypted input string to be decrypted.
/// * `key` - The shift value (number of positions each letter is moved in the alphabet).
///
/// # Returns
///
/// The decrypted version of the input string.
pub fn decrypt(ciphertext: &str, key: i32) -> String {
    encipher(Mode::Decrypt, ciphertext, key)
}

/// Encrypts or decrypts a given string based on the specified mode.
///
/// This is a helper function that performs the actual transformation of characters.
///
/// # Arguments
///
/// * `mode` - The operation mode (`Encrypt` or `Decrypt`).
/// * `data` - The input string to be processed.
/// * `key` - The shift value.
///
/// # Returns
///
/// The processed version of the input string based on the mode.
fn encipher(mode: Mode, data: &str, key: i32) -> String {
    data.chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                process_char(c, &mode, key, 'A')
            } else if c.is_ascii_lowercase() {
                process_char(c, &mode, key, 'a')
            } else {
                c
            }
        })
        .collect()
}

/// Transforms a character based on the mode and key.
///
/// This function is responsible for performing the Caesar cipher transformation for individual characters.
///
/// # Arguments
///
/// * `c` - The character to be processed.
/// * `mode` - The operation mode (`Encrypt` or `Decrypt`).
/// * `key` - The shift value.
/// * `offset` - The ASCII value offset (based on whether the character is uppercase or lowercase).
///
/// # Returns
///
/// The transformed character.
fn process_char(c: char, mode: &Mode, key: i32, offset: char) -> char {
    let char_pos = c as i32 - offset as i32;
    let encrypted_char_pos = match mode {
        Mode::Encrypt => (char_pos + key) % 26,
        Mode::Decrypt => (char_pos - key + 26) % 26,
    };
    (encrypted_char_pos + offset as i32) as u8 as char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar_encryption() {
        let plaintext = "Hello";
        let encrypted = encrypt(plaintext, 3);
        assert_eq!(encrypted, "Khoor");
    }

    #[test]
    fn test_caesar_decryption() {
        let ciphertext = "Khoor";
        let decrypted = decrypt(ciphertext, 3);
        assert_eq!(decrypted, "Hello");
    }

    #[test]
    fn test_caesar_encryption_and_decryption() {
        let plaintext = "HelloWorld";
        let encrypted = encrypt(plaintext, 3);
        assert_eq!(encrypted, "KhoorZruog");

        let decrypted = decrypt(&encrypted, 3);
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_caesar_encryption_and_decryption_non_alpha_chars() {
        let plaintext = "Hello, World!";
        let encrypted = encrypt(plaintext, 3);
        assert_eq!(encrypted, "Khoor, Zruog!");

        let decrypted = decrypt(&encrypted, 3);
        assert_eq!(decrypted, plaintext);
    }
}
