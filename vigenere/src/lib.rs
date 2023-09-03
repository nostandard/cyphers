//! An implementation of the Vigenère cipher

const ALPHABET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALPHABET_SIZE: usize = ALPHABET.len();

enum Operation {
    Encrypt,
    Decrypt,
}

/// Encrypts the provided plaintext using the Vigenère cipher and the given key.
pub fn encrypt(plaintext: &str, key: &str) -> String {
    let extended_key = extend_key(key, plaintext.len());

    plaintext
        .chars()
        .enumerate()
        .map(|(idx, c)| {
            let shift = ALPHABET
                .find(extended_key.chars().nth(idx).unwrap_or_default())
                .unwrap_or_default() as isize;
            char_shift(c, shift, Operation::Encrypt)
        })
        .collect()
}

/// Decrypts the provided ciphertext using the Vigenère cipher and the given key.
pub fn decrypt(ciphertext: &str, key: &str) -> String {
    let extended_key = extend_key(key, ciphertext.len());

    ciphertext
        .chars()
        .enumerate()
        .map(|(idx, char)| {
            let shift = ALPHABET
                .find(extended_key.chars().nth(idx).unwrap_or_default())
                .unwrap_or_default() as isize;
            char_shift(char, shift, Operation::Decrypt)
        })
        .collect()
}

/// Extends the key to match the desired length by repeating it as necessary.
fn extend_key(key: &str, txt_len: usize) -> String {
    key.chars().cycle().take(txt_len).collect()
}

/// Shifts a character in the alphabet by a specified amount, depending on the operation (encryption or decryption).
fn char_shift(c: char, shift: isize, op: Operation) -> char {
    let position = ALPHABET.find(c).unwrap_or_default() as isize;

    let new_position = match op {
        Operation::Encrypt => (position + shift).rem_euclid(ALPHABET_SIZE as isize),
        Operation::Decrypt => (position - shift).rem_euclid(ALPHABET_SIZE as isize),
    };

    ALPHABET.chars().nth(new_position as usize).unwrap_or(c)
}

#[doc(hidden)]
#[allow(dead_code)]
/// Prepares a string by converting it to uppercase and filtering out non-alphabetic characters.
fn prepare_string(input_string: &str) -> String {
    // Convert the string to uppercase and remove non-alphabetic characters
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
