//! An implementation of the ROT13 cipher

pub fn rot13(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() {
                    'a' as u8
                } else {
                    'A' as u8
                };
                let offset = (c as u8 - base + 13) % 26;
                (base + offset) as char
            } else {
                c
            }
        })
        .collect()
}
