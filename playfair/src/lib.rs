//! An implementation of the Playfair cipher.

use std::collections::HashSet;

enum Mode {
    Encrypt,
    Decrypt,
}

pub fn encrypt(keyword: &str, text: &str) -> Result<String, &'static str> {
    playfair_cipher(Mode::Encrypt, keyword, text)
}

pub fn decrypt(keyword: &str, text: &str) -> Result<String, &'static str> {
    playfair_cipher(Mode::Decrypt, keyword, text)
}

fn playfair_cipher(mode: Mode, keyword: &str, text: &str) -> Result<String, &'static str> {
    if keyword.is_empty() || text.is_empty() {
        return Err("Keyword and text cannot be empty");
    }

    let matrix = create_matrix(&keyword);
    let prepared_text = prepare_text(&text);
    let digraphs = create_digraphs(&prepared_text)?;

    let result_text = digraphs
        .iter()
        .map(|digraph| process_digraph(&matrix, digraph, &mode))
        .collect();

    Ok(result_text)
}

fn process_digraph(matrix: &[[char; 5]; 5], digraph: &(char, char), mode: &Mode) -> String {
    let (pos1, pos2) = (
        find_position(matrix, digraph.0),
        find_position(matrix, digraph.1),
    );

    let row_offset = match mode {
        Mode::Encrypt => 1,
        Mode::Decrypt => 4, // equivalent to -1 modulo 5
    };

    let col_offset = match mode {
        Mode::Encrypt => 1,
        Mode::Decrypt => 4, // equivalent to -1 modulo 5
    };

    if pos1.0 == pos2.0 {
        return format!(
            "{}{}",
            matrix[pos1.0][(pos1.1 + col_offset) % 5],
            matrix[pos2.0][(pos2.1 + col_offset) % 5]
        );
    }

    if pos1.1 == pos2.1 {
        return format!(
            "{}{}",
            matrix[(pos1.0 + row_offset) % 5][pos1.1],
            matrix[(pos2.0 + row_offset) % 5][pos2.1]
        );
    }

    format!("{}{}", matrix[pos1.0][pos2.1], matrix[pos2.0][pos1.1])
}

fn create_matrix(keyword: &str) -> [[char; 5]; 5] {
    let mut matrix = [[' '; 5]; 5];
    let mut alphabet: HashSet<char> = "ABCDEFGHIKLMNOPQRSTUVWXYZ".chars().collect();
    let mut key_chars: Vec<char> = keyword
        .to_uppercase()
        .chars()
        .filter(|c| alphabet.contains(c))
        .collect();
    key_chars.dedup();
    let mut idx = 0;

    for &char in key_chars.iter() {
        alphabet.remove(&char);
        matrix[idx / 5][idx % 5] = char;
        idx += 1;
    }

    for &char in alphabet.iter() {
        matrix[idx / 5][idx % 5] = char;
        idx += 1;
    }

    matrix
}

fn prepare_text(text: &str) -> String {
    text.to_uppercase()
        .replace("J", "I")
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect()
}

fn create_digraphs(text: &str) -> Result<Vec<(char, char)>, &'static str> {
    let mut digraphs = Vec::new();
    let mut iter = text.chars().peekable();

    while let Some(&first) = iter.peek() {
        iter.next();
        let second = iter.peek().cloned().unwrap_or('X');
        if second == first {
            digraphs.push((first, 'X'));
        } else {
            digraphs.push((first, second));
            iter.next();
        }
    }

    if digraphs.is_empty() {
        return Err("No valid characters in the input text");
    }

    Ok(digraphs)
}

fn find_position(matrix: &[[char; 5]; 5], ch: char) -> (usize, usize) {
    for (i, row) in matrix.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == ch {
                return (i, j);
            }
        }
    }
    (0, 0) // This line should never be reached because all input characters will be in the matrix
}
