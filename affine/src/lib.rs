//! An implementation of Affine cipher.

pub fn encrypt(plaintext: &str, a: i32, b: i32) -> Result<String, &'static str> {
    let mut ciphertext = String::new();
    for c in plaintext.chars() {
        if c.is_ascii_lowercase() {
            let x = c as i32 - 'a' as i32;
            let encrypted_char = ((a * x + b) % 26 + 26) % 26;
            ciphertext.push((encrypted_char + 'a' as i32) as u8 as char);
        } else {
            ciphertext.push(c);
        }
    }
    Ok(ciphertext)
}

pub fn decrypt(ciphertext: &str, a: i32, b: i32) -> Result<String, &'static str> {
    let inverse_a = modular_inverse(a, 26).ok_or("Failed to find modular inverse")?;
    let mut plaintext = String::new();
    for c in ciphertext.chars() {
        if c.is_ascii_lowercase() {
            let x = c as i32 - 'a' as i32;
            let decrypted_char = ((inverse_a * (x - b + 26)) % 26 + 26) % 26;
            plaintext.push((decrypted_char + 'a' as i32) as u8 as char);
        } else {
            plaintext.push(c);
        }
    }
    Ok(plaintext)
}

fn modular_inverse(a: i32, m: i32) -> Option<i32> {
    for i in 1..m {
        if (a * i) % m == 1 {
            return Some(i);
        }
    }
    None
}
