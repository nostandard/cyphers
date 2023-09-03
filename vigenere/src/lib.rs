//! An implementation of the Vigenère cipher

const ALPHABET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALPHABET_SIZE: usize = 26;

pub fn encrypt(plaintext: &str, key: &str) -> String {
    let extended_key = extend_key(key, plaintext.len());

    plaintext
        .chars()
        .enumerate()
        .map(|(idx, char)| {
            let shift = ALPHABET
                .find(extended_key.chars().nth(idx).unwrap_or_default())
                .unwrap_or_default() as isize;
            char_shift(char, shift, "encrypt")
        })
        .collect()
}

pub fn decrypt(ciphertext: &str, key: &str) -> String {
    let extended_key = extend_key(key, ciphertext.len());

    ciphertext
        .chars()
        .enumerate()
        .map(|(idx, char)| {
            let shift = ALPHABET
                .find(extended_key.chars().nth(idx).unwrap_or_default())
                .unwrap_or_default() as isize;
            char_shift(char, shift, "decrypt")
        })
        .collect()
}

// fn prepare_string(input_string: &str) -> String {
//     // Convert the string to uppercase and remove non-alphabetic characters
//     input_string
//         .chars()
//         .filter(|c| c.is_ascii_alphabetic())
//         .collect::<String>()
//         .to_uppercase()
// }

fn extend_key(key: &str, txt_len: usize) -> String {
    key.chars().cycle().take(txt_len).collect()
}

fn char_shift(c: char, shift: isize, operation: &str) -> char {
    let position = ALPHABET.find(c).unwrap_or_default() as isize;

    let new_position = match operation {
        "encrypt" => (position + shift).rem_euclid(ALPHABET_SIZE as isize),
        "decrypt" => (position - shift).rem_euclid(ALPHABET_SIZE as isize),
        _ => position,
    };

    ALPHABET.chars().nth(new_position as usize).unwrap_or(c)
}
