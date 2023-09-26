//! An implementation of the Porta cipher

#[derive(Clone, Copy)]
enum Mode {
    Encrypt,
    Decrypt,
}

pub fn encrypt(plaintext: &str, key: &str) -> String {
    encipher(plaintext, key, Mode::Encrypt)
}

pub fn decrypt(ciphertext: &str, key: &str) -> String {
    encipher(ciphertext, key, Mode::Decrypt)
}

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
                    let cipher_idx = if text_idx < 13 {
                        text_idx
                    } else {
                        text_idx - 13
                    };
                    index_to_char((cipher_idx + 26 - key_idx) % 13, false)
                }
            }
        })
        .collect()
}

fn prepare_key(text: &str, key: &str) -> String {
    let key = key.to_uppercase();
    key.chars().cycle().take(text.len()).collect()
}

fn char_to_index(ch: char) -> usize {
    (ch.to_ascii_uppercase() as usize) - ('A' as usize)
}

fn index_to_char(index: usize, is_second_half: bool) -> char {
    let adjusted_index = if is_second_half { index + 13 } else { index };
    char::from_u32((adjusted_index + 'A' as usize) as u32).unwrap()
}
