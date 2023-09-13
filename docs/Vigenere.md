# Vigenère Cipher

The Vigenère cipher is a method of encrypting alphabetic text by using a simple form of polyalphabetic substitution.

## How it works

1. Choose a keyword, which will be used as the key to encrypt and decrypt the message.
2. Repeat the keyword until it matches the length of the plaintext.
3. Add each letter of the plaintext to the corresponding letter of the keyword.
4. If the sum goes beyond 'Z' or the end of the alphabet, wrap around to the start of the alphabet.

### Example

Given the plaintext "HELLO" and the key "KEY", the key is repeated to match the length of the plaintext, resulting in "KEYKE". Now, each letter of "HELLO" is shifted by the corresponding letter in "KEYKE".

Decryption is the reverse process.

## Algorithm

1. ***Preparation***:

    * Start with your plaintext message that you want to encrypt and a keyword (the key) to use for encryption.
    * Convert both the plaintext message and the key to uppercase. This ensures consistency since the Vigenère cipher typically works with a single case.
    * Remove any spaces or non-alphabetic characters from the plaintext message and the key. This ensures that we're only working with letters.

2. ***Key Extension***:

    * If the key is shorter than the plaintext, repeat the key until it matches the length of the plaintext.
      * For example, if the plaintext is "HELLO" and the key is "KEY", the extended key becomes "KEYKE".

3. ***Encryption***:

    * For each letter in the plaintext:
      * Find its position in the alphabet. Let's call this position *P*.
      * Find the position in the alphabet of the corresponding letter in the extended key. Let's call this position *K*.
      * Add *P* and *K* together to get a new position. If this position is beyond the end of the alphabet, wrap around to the beginning. This new position corresponds to the encrypted letter.
      * Replace the plaintext letter with the encrypted letter from the new position.
    * The result, after all the letters in the plaintext have been replaced, is the encrypted message or ciphertext.

4. ***Decryption***:

    * For each letter in the ciphertext:
      * Find its position in the alphabet. Let's call this position *C*.
      * Find the position in the alphabet of the corresponding letter in the extended key. Let's call this position *K*.
      * Subtract *K* from *C* to get the original position (i.e., *C - K*). If this position is negative, add the length of the alphabet to get a position in the range of the alphabet. This original position corresponds to the decrypted letter.
      * Replace the ciphertext letter with the decrypted letter from the original position.
    * The result, after all the letters in the ciphertext have been replaced, is the decrypted message or original plaintext.

## Possible Attacks

1. Frequency Analysis:

    * Even though the Vigenère cipher is more secure compared to simple substitution ciphers, it is still susceptible to frequency analysis. If the key length is known or can be guessed, frequency analysis can be applied to each subset of letters encrypted with the same key letter.

2. Kasiski Examination:

    * A method which involves finding repeating sequences of characters in the ciphertext, determining the distances between them, and finding the greatest common divisor of the distances to estimate the length of the key.

3. Key Elimination:

    * If the length of the key is known, this method can be used to decipher the text by using the key to decrypt the text, effectively reducing it to a simple `Caesar` cipher, which can then be broken easily.

4. Brute-Force Attack:

    * Trying all possible keys to decrypt the ciphertext. This is computationally intensive and time-consuming, but becomes feasible for short keys.

5. Dictionary Attack:

    * If the key is a known common word, a dictionary attack where a pre-compiled list of words (dictionary) is used to guess the key could be effective.
