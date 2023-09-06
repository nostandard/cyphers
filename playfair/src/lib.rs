//! An implementation of the Playfair cipher.

use std::collections::HashSet;
use thiserror::Error;

/// Error type for the Playfair cipher functions.
#[derive(Error, Debug)]
pub enum PlayfairError {
    #[error("Keyword and text cannot be empty")]
    EmptyInput,
    #[error("No valid characters in the input text")]
    InvalidText,
    #[error("Character not found in matrix")]
    CharNotFound,
}

/// Modes of operation for the Playfair cipher.
#[derive(Clone, Copy)]
enum Mode {
    Encrypt,
    Decrypt,
}

/// Encrypts a given plaintext string using the Playfair cipher.
///
/// # Arguments
///
/// * `keyword` - The key to be used for creating the matrix.
/// * `text` - The input string to be encrypted.
///
/// # Returns
///
/// The encrypted version of the input string.
pub fn encrypt(keyword: &str, text: &str) -> Result<String, PlayfairError> {
    playfair_cipher(Mode::Encrypt, keyword, text)
}

/// Decrypts a given ciphertext string using the Playfair cipher.
///
/// # Arguments
///
/// * `keyword` - The key to be used for creating the matrix.
/// * `text` - The encrypted input string to be decrypted.
///
/// # Returns
///
/// The decrypted version of the input string.
pub fn decrypt(keyword: &str, text: &str) -> Result<String, PlayfairError> {
    playfair_cipher(Mode::Decrypt, keyword, text)
}

/// Encrypts or decrypts a given string based on the specified mode using the Playfair cipher.
///
/// This is a helper function that performs the actual transformation of characters.
///
/// # Arguments
///
/// * `mode` - The operation mode (`Encrypt` or `Decrypt`).
/// * `keyword` - The key to create the matrix.
/// * `text` - The input string to be processed.
///
/// # Returns
///
/// The processed version of the input string based on the mode.
fn playfair_cipher(mode: Mode, keyword: &str, text: &str) -> Result<String, PlayfairError> {
    if keyword.is_empty() || text.is_empty() {
        return Err(PlayfairError::EmptyInput);
    }

    let matrix = create_matrix(keyword);
    let prepared_text = prepare_text(text);
    let digraphs = create_digraphs(&prepared_text)?;

    let result_text: Result<String, _> = digraphs
        .iter()
        .map(|digraph| process_digraph(&matrix, digraph, &mode))
        .collect();

    result_text
}

/// Creates a 5x5 matrix for the Playfair cipher using the given keyword.
///
/// # Arguments
///
/// * `keyword` - The keyword used to create the matrix.
///
/// # Returns
///
/// A 5x5 matrix filled with characters.
fn create_matrix(keyword: &str) -> [[char; 5]; 5] {
    let mut matrix = [[' '; 5]; 5];
    let mut alphabet: HashSet<char> = "ABCDEFGHIKLMNOPQRSTUVWXYZ".chars().collect();
    let mut key_chars: Vec<char> = keyword
        .to_uppercase()
        .chars()
        .filter(|c| alphabet.contains(c))
        .collect();
    key_chars.dedup();

    for (idx, &char) in key_chars.iter().enumerate() {
        alphabet.remove(&char);
        matrix[idx / 5][idx % 5] = char;
    }

    for (idx, &char) in alphabet.iter().enumerate() {
        matrix[(key_chars.len() + idx) / 5][(key_chars.len() + idx) % 5] = char;
    }

    matrix
}

/// Prepares the input text for the Playfair cipher.
///
/// # Arguments
///
/// * `text` - The input text to be prepared.
///
/// # Returns
///
/// A string with 'J' replaced by 'I' and non-alphabetic characters removed.
fn prepare_text(text: &str) -> String {
    text.to_uppercase()
        .replace("J", "I")
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect()
}

/// Creates digraphs from the input text.
///
/// # Arguments
///
/// * `text` - The input text to be converted into digraphs.
///
/// # Returns
///
/// A vector of digraph tuples or an error if there are no valid characters in the input text.
fn create_digraphs(text: &str) -> Result<Vec<(char, char)>, PlayfairError> {
    let mut digraphs = Vec::new();
    let chars: Vec<_> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let first = chars[i];
        let second = if i + 1 < chars.len() && chars[i + 1] != first {
            i += 1;
            chars[i]
        } else {
            'X'
        };

        digraphs.push((first, second));
        i += 1;
    }

    if digraphs.is_empty() {
        Err(PlayfairError::InvalidText)
    } else {
        Ok(digraphs)
    }
}

/// Finds the position of a character in the matrix.
///
/// # Arguments
///
/// * `matrix` - The 5x5 matrix to search.
/// * `ch` - The character to find.
///
/// # Returns
///
/// The position of the character in the matrix or an error if the character is not found.
fn find_position(matrix: &[[char; 5]; 5], ch: char) -> Result<(usize, usize), PlayfairError> {
    for (i, row) in matrix.iter().enumerate() {
        for (j, &char) in row.iter().enumerate() {
            if char == ch {
                return Ok((i, j));
            }
        }
    }

    Err(PlayfairError::CharNotFound)
}

/// Processes a single digraph according to the Playfair cipher rules.
///
/// # Arguments
///
/// * `matrix` - The 5x5 matrix used for the cipher.
/// * `digraph` - The digraph tuple to be processed.
/// * `mode` - The operation mode (`Encrypt` or `Decrypt`).
///
/// # Returns
///
/// A string representing the processed digraph or an error if any character is not found in the matrix.
fn process_digraph(
    matrix: &[[char; 5]; 5],
    digraph: &(char, char),
    mode: &Mode,
) -> Result<String, PlayfairError> {
    let (x1, y1) = find_position(matrix, digraph.0)?;
    let (x2, y2) = find_position(matrix, digraph.1)?;

    if x1 == x2 {
        let new_y1 = match mode {
            Mode::Encrypt => (y1 + 1) % 5,
            Mode::Decrypt => (y1 + 4) % 5,
        };
        let new_y2 = match mode {
            Mode::Encrypt => (y2 + 1) % 5,
            Mode::Decrypt => (y2 + 4) % 5,
        };
        Ok(format!("{}{}", matrix[x1][new_y1], matrix[x2][new_y2]))
    } else if y1 == y2 {
        let new_x1 = match mode {
            Mode::Encrypt => (x1 + 1) % 5,
            Mode::Decrypt => (x1 + 4) % 5,
        };
        let new_x2 = match mode {
            Mode::Encrypt => (x2 + 1) % 5,
            Mode::Decrypt => (x2 + 4) % 5,
        };
        Ok(format!("{}{}", matrix[new_x1][y1], matrix[new_x2][y2]))
    } else {
        Ok(format!("{}{}", matrix[x1][y2], matrix[x2][y1]))
    }
}
