//! An implementation of the Caesar cipher.

enum Mode {
    Encrypt,
    Decrypt,
}

pub fn encrypt(plaintext: &str, key: i32) -> String {
    encipher(Mode::Encrypt, plaintext, key)
}

pub fn decrypt(ciphertext: &str, key: i32) -> String {
    encipher(Mode::Decrypt, ciphertext, key)
}

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
