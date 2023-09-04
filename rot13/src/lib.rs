//! An implementation of the ROT13 cipher

const ROTATION: u8 = 13;
const ALPHABET_SIZE: u8 = 26;

/// Performs a ROT13 transformation on a given string slice.
///
/// The ROT13 transformation is performed on each alphabetic character in the input string.
/// Non-alphabetic characters are left unchanged.
///
/// # Examples
///
/// ```
/// use rot13::rot13;
///
/// assert_eq!(rot13("Hello, World!"), "Uryyb, Jbeyq!");
/// ```
pub fn rot13(input: &str) -> String {
    input
        .chars()
        // Iterate over each character in the input string.
        .map(|c| {
            if c.is_ascii_alphabetic() {
                // Determine the ASCII value of the base character ('a' for lowercase, 'A' for uppercase).
                let base = if c.is_ascii_lowercase() {
                    'a' as u8
                } else {
                    'A' as u8
                };
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
