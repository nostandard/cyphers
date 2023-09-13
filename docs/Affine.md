# Affine Cipher

The Affine cipher is a type of monoalphabetic substitution cipher. In this cipher, each letter in an alphabet is mapped to its numeric equivalent, encrypted using a simple mathematical function, and then converted back to a letter. The mathematical encryption function used in the Affine cipher is a linear equation given by:
*E(x) = (ax + b) mod m*

and the decryption function is given by:
*D(x) = a<sup>-1</sup>(x - b) mod m*

Where:

* *E(x)* is the encryption function
* *D(x)* is the decryption function
* *x* is the numerical equivalent of a letter (0 for 'a', 1 for 'b', ..., 25 for 'z')
* *a* and *b* are keys chosen such that *a* is coprime with *m*
* *m* is the size of the alphabet (26 for English letters)
* *a<sup>-1</sup>* is the modular inverse of *a* modulo *m*

## Algorithm

1. ***Key Validation***:
    * Choose two keys: *a* (must be coprime with 26) and *b* (any integer).
    * Validate key *a*: check if the greatest common divisor (GCD) of *a* and 26 is 1 (which means they are coprime). If not, prompt the user to enter a valid key.

2. ***Encryption***:
    * Initialize an empty string to hold the ciphertext.
    * Loop through each character in the plaintext.
    * If the character is a lowercase letter, proceed to the next step. If not, add it to the ciphertext as is.
    * Convert the character to its numerical representation.
    * Use the formula *E(x) = (ax + b) mod 26* to find the encrypted numerical value.
    * Convert the numerical value back to a character.
    * Add the encrypted character to the ciphertext.

3. ***Decryption***:
    * Initialize an empty string to hold the decrypted plaintext.
    * Loop through each character in the ciphertext.
    * If the character is a lowercase letter, proceed to the next step. If not, add it to the plaintext as is.
    * Convert the character to its numerical representation.
    * Find the modular inverse of *a*.
    * Use the formula *D(x) = a<sup>−1</sup>(x − b) mod 26* to find the decrypted numerical value. Here, *a<sup>-1</sup>* is the modular inverse found in the previous step.
    * Convert the numerical value back to a character.
    * Add the decrypted character to the plaintext.

## Possible Attacks

1. Brute Force Attack:

    * Since the key space of the affine cipher is small (key a has 12 possible values, and key b has 26 possible values, making for 312 possible key pairs), an attacker can try all possible key pairs to decrypt the ciphertext.

2. Known-plaintext Attack:

    * If an attacker has a piece of plaintext and its corresponding ciphertext, they can derive the keys a and b by solving the simultaneous equations generated from the encryption formula.

3. Frequency Analysis:

    * The Affine cipher is a monoalphabetic substitution cipher, which means that each letter in the plaintext is always mapped to the same letter in the ciphertext. This property allows an attacker to use frequency analysis, where they analyze the frequencies of letters in the ciphertext and match them to the expected frequencies of letters in the English language (or any other relevant language) to find the keys.

4. Chosen-plaintext Attack:

    * An attacker could potentially choose specific pieces of plaintext to encrypt (using the encryption mechanism) and analyze the resulting ciphertexts to derive the keys a and b.
