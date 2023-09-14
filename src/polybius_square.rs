//! An implementation of the Polybius Square cipher.

/// The grid used for encryption and decryption.
const GRID: [[char; 5]; 5] = [
    ['A', 'B', 'C', 'D', 'E'],
    ['F', 'G', 'H', 'I', 'K'],
    ['L', 'M', 'N', 'O', 'P'],
    ['Q', 'R', 'S', 'T', 'U'],
    ['V', 'W', 'X', 'Y', 'Z'],
];

/// Encrypts a plaintext message using the Polybius Square cipher.
///
/// # Arguments
///
/// * `plaintext` - The message to encrypt.
///
/// # Returns
///
/// * A string containing the ciphertext.
pub fn encrypt(plaintext: &str) -> String {
    let cleaned_input = clean_input(plaintext);
    cleaned_input
        .chars()
        .filter_map(|ch| find_coordinates(ch).map(|(row, col)| format!("{}{}", row + 1, col + 1)))
        .collect()
}

/// Decrypts a ciphertext message using the Polybius Square cipher.
///
/// # Arguments
///
/// * `ciphertext` - The message to decrypt.
///
/// # Returns
///
/// * An option containing a string with the decrypted message if successful, or `None` if the input is invalid.
pub fn decrypt(ciphertext: &str) -> Option<String> {
    if ciphertext.len() % 2 != 0 {
        return None;
    }

    let mut plaintext = String::new();
    for chunk in ciphertext.as_bytes().chunks(2) {
        if chunk.len() == 2 {
            let coordinates = (
                (chunk[0] as char).to_digit(10)? as usize - 1,
                (chunk[1] as char).to_digit(10)? as usize - 1,
            );
            let ch = find_char(coordinates)?;
            plaintext.push(ch);
        }
    }
    Some(plaintext)
}

/// Cleans the input string by converting to uppercase and filtering out invalid characters.
///
/// # Arguments
///
/// * `input` - The string to clean.
///
/// # Returns
///
/// * A string containing only valid characters in uppercase.
fn clean_input(input: &str) -> String {
    input
        .to_uppercase()
        .chars()
        .filter(|&c| c >= 'A' && c <= 'Z' && c != 'J')
        .collect()
}

/// Finds the coordinates of a character in the grid.
///
/// # Arguments
///
/// * `ch` - The character to find.
///
/// # Returns
///
/// * An option containing a tuple with the coordinates (row, col) if the character is found, or `None` otherwise.
fn find_coordinates(ch: char) -> Option<(usize, usize)> {
    for (i, row) in GRID.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == ch {
                return Some((i, j));
            }
        }
    }
    None
}

/// Finds the character in the grid at the given coordinates.
///
/// # Arguments
///
/// * `coordinates` - A tuple containing the row and column indices.
///
/// # Returns
///
/// * An option containing the character if the coordinates are valid, or `None` otherwise.
fn find_char(coordinates: (usize, usize)) -> Option<char> {
    if coordinates.0 < 5 && coordinates.1 < 5 {
        Some(GRID[coordinates.0][coordinates.1])
    } else {
        None
    }
}
