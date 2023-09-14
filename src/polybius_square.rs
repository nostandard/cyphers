//! An implementation of the Polybius Square cipher.

/// The uppercase-alphabetic grid used for encryption and decryption.
const GRID_UPPER: [[char; 5]; 5] = [
    ['A', 'B', 'C', 'D', 'E'],
    ['F', 'G', 'H', 'I', 'K'],
    ['L', 'M', 'N', 'O', 'P'],
    ['Q', 'R', 'S', 'T', 'U'],
    ['V', 'W', 'X', 'Y', 'Z'],
];

/// The lowercase-alphabetic grid used for encryption and decryption.
const GRID_LOWER: [[char; 5]; 5] = [
    ['a', 'b', 'c', 'd', 'e'],
    ['f', 'g', 'h', 'i', 'k'],
    ['l', 'm', 'n', 'o', 'p'],
    ['q', 'r', 's', 't', 'u'],
    ['v', 'w', 'x', 'y', 'z'],
];

/// Encrypts a plaintext using the Polybius Square cipher.
///
/// # Arguments
///
/// * `plaintext` - A reference to a string slice containing the text to be encrypted.
///
/// # Returns
///
/// * A `String` holding the encrypted text. White spaces in the plaintext are preserved.
pub fn encrypt(plaintext: &str) -> String {
    let cleaned_input = clean_input(plaintext);
    cleaned_input
        .chars()
        .map(|ch| {
            if ch == ' ' {
                String::from(" ")
            } else {
                find_coordinates(ch)
                    .map(|(row, col, _)| format!("{}{}", row + 1, col + 1))
                    .unwrap_or_default()
            }
        })
        .collect()
}

/// Decrypts a ciphertext using the Polybius Square cipher.
///
/// # Arguments
///
/// * `ciphertext` - A reference to a string slice containing the text to be decrypted.
///
/// # Returns
///
/// * An `Option<String>` holding the decrypted text. If the ciphertext is invalid, it returns None.
///   White spaces in the ciphertext are preserved.
pub fn decrypt(ciphertext: &str) -> Option<String> {
    let mut plaintext = String::new();
    let mut buffer = String::new();

    for ch in ciphertext.chars() {
        if ch == ' ' {
            if buffer.len() == 2 {
                if let Some(decoded_char) = decode_buffer(&buffer) {
                    plaintext.push(decoded_char);
                    buffer.clear();
                } else {
                    return None;
                }
            }
            plaintext.push(' ');
        } else {
            buffer.push(ch);
            if buffer.len() == 2 {
                if let Some(decoded_char) = decode_buffer(&buffer) {
                    plaintext.push(decoded_char);
                    buffer.clear();
                } else {
                    return None;
                }
            }
        }
    }

    if buffer.len() == 2 {
        if let Some(decoded_char) = decode_buffer(&buffer) {
            plaintext.push(decoded_char);
        } else {
            return None;
        }
    } else if !buffer.is_empty() {
        return None;
    }

    Some(plaintext)
}

/// Cleans the input string by converting it to uppercase and replacing 'J' with 'I'.
/// It also preserves spaces for word separation.
///
/// # Arguments
///
/// * `input` - A reference to a string slice that needs to be cleaned.
///
/// # Returns
///
/// * A `String` with cleaned-up input, ready to be used for encryption or decryption.
fn clean_input(input: &str) -> String {
    input
        .chars()
        .filter_map(|c| {
            if c.is_alphabetic() {
                if c == 'J' {
                    Some('I')
                } else if c == 'j' {
                    Some('i')
                } else {
                    Some(c)
                }
            } else if c == ' ' {
                Some(c)
            } else {
                None
            }
        })
        .collect()
}

/// Decodes a buffer of two characters to a character from the Polybius square.
///
/// # Arguments
///
/// * `buffer` - A reference to a string slice containing two digits representing a coordinate in the Polybius square.
///
/// # Returns
///
/// * An `Option<char>` containing the decrypted character corresponding to the coordinates in the buffer.
///   If the buffer contains invalid coordinates, it returns None.
fn decode_buffer(buffer: &str) -> Option<char> {
    let coordinates = (
        buffer.chars().next()?.to_digit(10)? as usize - 1,
        buffer.chars().next()?.to_digit(10)? as usize - 1,
    );

    find_char(coordinates, 'L').or_else(|| find_char(coordinates, 'U'))
}

/// Finds the coordinates of a character in the grids.
///
/// # Arguments
///
/// * `ch` - The character to find.
///
/// # Returns
///
/// * An option containing a tuple with the coordinates (row, col) and a character
/// indicating the grid ('U' for uppercase and 'L' for lowercase)
/// where the character was found, or `None` otherwise.
fn find_coordinates(ch: char) -> Option<(usize, usize, char)> {
    for (i, row) in GRID_UPPER.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == ch {
                return Some((i, j, 'U'));
            }
        }
    }
    for (i, row) in GRID_LOWER.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == ch {
                return Some((i, j, 'L'));
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
/// * `grid_indicator` - A char indicating which grid to use ('U' for uppercase and 'L' for lowercase).
///
/// # Returns
///
/// * An option containing the character if the coordinates are valid, or `None` otherwise.
fn find_char(coordinates: (usize, usize), grid_indicator: char) -> Option<char> {
    if coordinates.0 < 5 && coordinates.1 < 5 {
        match grid_indicator {
            'U' => GRID_UPPER
                .get(coordinates.0)
                .and_then(|row| row.get(coordinates.1))
                .copied(),
            'L' => GRID_LOWER
                .get(coordinates.0)
                .and_then(|row| row.get(coordinates.1))
                .copied(),
            _ => None,
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polybius_square_encrypt() {
        assert_eq!(encrypt("Hello World"), "2315313134 5234423114");
        assert_eq!(encrypt("Polybius Square"), "3534315412244543 434145114215");
        assert_eq!(encrypt("RUST"), "42454344");
    }

    // #[test]
    // fn test_polybius_square_decrypt() {
    //     assert_eq!(
    //         decrypt("2315313134 5234423114"),
    //         Some("Hello World".to_string())
    //     );
    //     assert_eq!(
    //         decrypt("35343414424343 434144311514"),
    //         Some("polybius square".to_string())
    //     );
    //     assert_eq!(decrypt("42454344"), Some("RUST".to_string()));
    //     assert_eq!(decrypt("424543441"), None);
    // }

    #[test]
    fn test_polybius_square_clean_input() {
        assert_eq!(clean_input("Hello% World!"), "Hello World");
        assert_eq!(
            clean_input("Polybius???????? Square%£@!±&*()"),
            "Polybius Square"
        );
        assert_eq!(
            clean_input("RUST Programming Language"),
            "RUST Programming Language"
        );
    }

    #[test]
    fn test_polybius_square_find_coordinates() {
        assert_eq!(find_coordinates('H'), Some((1, 2, 'U')));
        assert_eq!(find_coordinates('e'), Some((0, 4, 'L')));
        assert_eq!(find_coordinates('l'), Some((2, 0, 'L')));
        assert_eq!(find_coordinates('o'), Some((2, 3, 'L')));
        assert_eq!(find_coordinates('W'), Some((4, 1, 'U')));
    }

    #[test]
    fn test_polybius_square_find_char() {
        assert_eq!(find_char((1, 2), 'U'), Some('H'));
        assert_eq!(find_char((0, 4), 'L'), Some('e'));
        assert_eq!(find_char((2, 0), 'L'), Some('l'));
        assert_eq!(find_char((2, 3), 'L'), Some('o'));
        assert_eq!(find_char((4, 1), 'U'), Some('W'));
        assert_eq!(find_char((5, 5), 'U'), None);
    }
}
