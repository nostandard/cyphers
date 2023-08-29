# Caesar Cipher

## Overview

The Caesar cipher is a type of substitution cipher where each letter in the plaintext is shifted a certain number of places down or up the alphabet. For instance, with a shift of 1, 'A' would be replaced by 'B', 'B' would become 'C', and so on. The method is named after Julius Caesar, who reportedly used it to communicate with his generals.

The formula to encrypt a letter using a Caesar cipher is:

E<sub>n</sub>(x) = (x + n) mod 26

Where:

* E<sub>n</sub>(x) is the encrypted letter
* x is the plaintext letter
* n is the shift value

The formula to decrypt is:

D<sub>n</sub>(x) = (x - n) mod 26

## Algorithm

1. ***Setup:***

* Start with the alphabet: A, B, C, ... Z.
* Assign each letter a position: A=0, B=1, C=2, ..., Z=25.

2. ***Gather Input:***

* `MODE`: Ask the user if they want to encrypt or decrypt.
* `DATA`: Ask the user for the message they want to process.
* `KEY`: Ask the user for the shift value (how many positions each letter should be moved).

3. ***Prepare the Message:***

If there are any non-alphabetical characters in the message, just leave them as they are during processing.

4. ***Process Each Character:***

* For each character in the message:
  * Determine if it's an uppercase letter, a lowercase letter, or neither.
  * If it's a letter (A-Z or a-z):
    * Convert the letter to its position in the alphabet (0-25). For example, A becomes 0, B becomes 1, and so on.
  * If you're *encrypting*:
    * Add the shift value to the position.
    * If the result is 26 or larger, wrap it around to the beginning. This is done by taking the result modulo 26.
    * Convert the new position back to its corresponding letter.
  * If you're *decrypting*:
    * Subtract the shift value from the position.
    * If the result is negative, add 26 to make it positive and wrap it around to the end of the alphabet.
    * Then, take the result modulo 26 to ensure it's within the 0-25 range.
    * Convert the new position back to its corresponding letter.
  * If it's not a letter (e.g., punctuation, number):
    * Leave it unchanged.

5. ***Compile the Result:***

Collect all the processed characters to form the new message, preserving the order.

6. ***Output:***

Display the new, encrypted or decrypted message to the user.
