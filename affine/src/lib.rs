//! An implementation of the Affine cipher.

/// Encrypts a given plaintext using the Affine cipher.
///
/// # Arguments
///
/// * `plaintext` - The text to be encrypted.
/// * `a` - The multiplicative key. Should be coprime with 26.
/// * `b` - The additive key. Can be any integer.
///
/// # Returns
///
/// * `Result<String, &'static str>` - Returns an Ok variant with encrypted text as a string,
/// or an Err variant with a static string describing the error.
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

/// Decrypts a given ciphertext using the Affine cipher.
///
/// # Arguments
///
/// * `ciphertext` - The text to be decrypted.
/// * `a` - The multiplicative key. Should be coprime with 26.
/// * `b` - The additive key. Can be any integer.
///
/// # Returns
///
/// * `Result<String, &'static str>` - Returns an Ok variant with decrypted text as a string,
/// or an Err variant with a static string describing the error.
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

/// Finds the modular inverse of a given number modulo `m`.
///
/// # Arguments
///
/// * `a` - The number for which the modular inverse needs to be found. Should be coprime with `m`.
/// * `m` - The modulo.
///
/// # Returns
///
/// * `Option<i32>` - Returns a Some variant with the modular inverse if it exists,
/// or a None variant if it doesn't exist.
fn modular_inverse(a: i32, m: i32) -> Option<i32> {
    for i in 1..m {
        if (a * i) % m == 1 {
            return Some(i);
        }
    }
    None
}
