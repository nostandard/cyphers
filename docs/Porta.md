# Porta Cipher

The Porta cipher is a polyalphabetic substitution cipher that uses a simple table and a keyword to encipher a message. It was invented by Giovanni Battista Della Porta.

The key to the Porta cipher is a word (e.g., KEYWORD). The table used is just the English alphabet listed twice in sequence.

***For encryption***:

1. The plaintext letter and a letter from the key determine the ciphertext letter.
2. We take the sum of the positions of these letters in the alphabet (starting from 0), modulo 13 (since there are 26 letters and we're working with two sets of them).
3. We then take the letter from the second set of the alphabet that matches this position as the ciphertext letter.

***For decryption***:

1. We subtract, rather than add, the key letter's position from the ciphertext letter's position in the second half of the table, then modulo 13.
2. We then take the letter from the first set of the alphabet that matches this position as the plaintext letter.

## Algorithm

1. **Key Setup**
    * Begin by choosing a keyword. This will be the secret key to encrypt and decrypt messages.
    * Convert all letters in the keyword to uppercase. Ignore numbers, symbols, or other non-alphabetic characters; only the English alphabet (A-Z) matters.
    * If your message (plaintext for encryption or ciphertext for decryption) is longer than your keyword, repeat the keyword over and over until the repeated keyword matches the length of your message.

2. **Encryption**:
    * For each character in the plaintext:
        * Convert it into a number based on its position in the English alphabet. For example, 'A' becomes 0, 'B' becomes 1, and so on, until 'Z' which becomes 25.
        * Convert the corresponding character from the repeated keyword into a number in the same manner.

    * For each pair of numbers (one from the plaintext and one from the keyword):
        * Add the two numbers together.
        * Apply a modulus operation with the value of 13 to the sum. This ensures the result is always between 0 and 12.
        * Translate the result into a character from the second half of the English alphabet. So a 0 translates to 'N', 1 to 'O', and so on, until 12 which translates to 'Z'.
        * Append this character to the encrypted message (ciphertext).

3. **Decryption**:
    * For each character in the ciphertext:
        * Convert it into a number based on its position in the English alphabet, keeping in mind this time it's from the second half. So, 'N' becomes 0, 'O' becomes 1, and so on, until 'Z' which becomes 12.
        * Convert the corresponding character from the repeated keyword into a number based on its position in the full English alphabet, just as in the encryption process.

    * For each pair of numbers (one from the ciphertext and one from the keyword):
        * Subtract the number of the keyword character from the number of the ciphertext character.
        * Since we're dealing with the second half of the alphabet, to avoid negative numbers, first add 13 to the result.
        * Apply a modulus operation with the value of 13 to this sum. This ensures the result is always between 0 and 12.
        * Translate the result into a character from the first half of the English alphabet. So a 0 translates to 'A', 1 to 'B', and so on, until 12 which translates to 'M'.
        * Append this character to the decrypted message (plaintext).
