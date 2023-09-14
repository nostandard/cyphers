//! An implementation of Polybius Square cipher

const GRID: [[char; 5]; 5] = [
    ['A', 'B', 'C', 'D', 'E'],
    ['F', 'G', 'H', 'I', 'K'],
    ['L', 'M', 'N', 'O', 'P'],
    ['Q', 'R', 'S', 'T', 'U'],
    ['V', 'W', 'X', 'Y', 'Z'],
];

pub fn encrypt(plaintext: &str) -> String {
    let cleaned_input = clean_input(plaintext);
    cleaned_input
        .chars()
        .filter_map(|ch| find_coordinates(ch).map(|(row, col)| format!("{}{}", row + 1, col + 1)))
        .collect()
}

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

fn clean_input(input: &str) -> String {
    input
        .to_uppercase()
        .chars()
        .filter(|&c| c >= 'A' && c <= 'Z' && c != 'J')
        .collect()
}

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

fn find_char(coordinates: (usize, usize)) -> Option<char> {
    if coordinates.0 < 5 && coordinates.1 < 5 {
        Some(GRID[coordinates.0][coordinates.1])
    } else {
        None
    }
}
