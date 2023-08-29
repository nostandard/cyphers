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
