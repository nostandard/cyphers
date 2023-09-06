//! An implementation of the ROT13 cipher

const ROTATION: u8 = 13;
const ALPHABET_SIZE: u8 = 26;

/// Performs a ROT13 transformation on a given string slice.
///
/// The ROT13 transformation is performed on each alphabetic character in the input string.
/// Non-alphabetic characters are left unchanged.
///
/// It is worth noting that ROT13 is its own inverse; applying it twice will get you back the original input.
/// Thus, this function is used both to encrypt plaintext to ciphertext, and to decrypt ciphertext to plaintext.
///
/// # Examples
///
/// ```
/// use rot13::rot13;
///
/// assert_eq!(rot13("Hello, World!"), "Uryyb, Jbeyq!");
/// assert_eq!(rot13("Uryyb, Jbeyq!"), "Hello, World!");
/// ```
pub fn rot13(input: &str) -> String {
    input
        .chars()
        // Iterate over each character in the input string.
        .map(|c| {
            if c.is_ascii_alphabetic() {
                // Determine the ASCII value of the base character ('a' for lowercase, 'A' for uppercase).
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                // Calculate the ROT13 offset for the current character.
                let offset = (c as u8 - base + ROTATION) % ALPHABET_SIZE;
                // Convert the offset back to a char and return it.
                (base + offset) as char
            } else {
                // If the character is not an ASCII alphabetic character, leave it unchanged.
                c
            }
        })
        // Collect the modified characters into a new string and return it.
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot13_with_lowercase() {
        assert_eq!(
            rot13("abcdefghijklmnopqrstuvwxyz"),
            "nopqrstuvwxyzabcdefghijklm"
        );
    }

    #[test]
    fn test_rot13_with_uppercase() {
        assert_eq!(
            rot13("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            "NOPQRSTUVWXYZABCDEFGHIJKLM"
        );
    }

    #[test]
    fn test_rot13_with_mixed_case() {
        assert_eq!(rot13("Hello, World!"), "Uryyb, Jbeyq!");
    }

    #[test]
    fn test_rot13_with_non_alphabetic() {
        assert_eq!(rot13("12345!@#$%^"), "12345!@#$%^");
    }

    #[test]
    fn test_rot13_idempotent() {
        let input = "ROT13 is Idempotent!";
        assert_eq!(rot13(&rot13(input)), input);
    }
}
