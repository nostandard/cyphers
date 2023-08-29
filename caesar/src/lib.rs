//! An implementation of the Caesar cipher.

pub fn encrypt(plaintext: &str, key: i32) -> String {
    encipher("encrypt", plaintext, key)
}

pub fn decrypt(ciphertext: &str, key: i32) -> String {
    encipher("decrypt", ciphertext, key)
}

fn encipher(mode: &str, data: &str, key: i32) -> String {
    let mut result = String::new();

    for c in data.chars() {
        if c.is_ascii_uppercase() {
            result.push(process_char(c, mode, key, true));
        } else if c.is_ascii_lowercase() {
            result.push(process_char(c, mode, key, false));
        } else {
            result.push(c);
        }
    }

    result
}

fn process_char(c: char, mode: &str, key: i32, is_uppercase: bool) -> char {
    let mut char_pos = 0;
    let mut encrypted_char_pos = 0;

    if is_uppercase {
        char_pos = char_to_num(c, 'A');
    } else {
        char_pos = char_to_num(c, 'a');
    }

    if mode == "encrypt" {
        encrypted_char_pos = (char_pos + key) % 26;
    } else if mode == "decrypt" {
        encrypted_char_pos = (char_pos - key + 26) % 26;
    }

    if is_uppercase {
        return num_to_char(encrypted_char_pos, 'A');
    } else {
        return num_to_char(encrypted_char_pos, 'a');
    }
}

fn char_to_num(c: char, offset: char) -> i32 {
    c as i32 - offset as i32
}

fn num_to_char(n: i32, offset: char) -> char {
    (n + offset as i32) as u8 as char
}
