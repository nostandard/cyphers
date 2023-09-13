# ROT13

`ROT13` (Rotate by 13 places) is a simple letter substitution cipher that replaces a letter with the 13th letter after it in the alphabet. ROT13 is a special case of the `Caesar` cipher, which was developed in ancient Rome. In this cipher:

*A → N*
*B → O*
*C → P*
...
*N → A*
*O → B*
...
*Z → M*

The transformation wraps around the alphabet, so after "Z", it goes back to "A". It is worth noting that ROT13 is its own inverse; applying it twice will get you back the original input.

## Algorithm

1. Define a function that takes a string as input and returns a modified string.

2. Create a mutable string variable, say `result``, to hold the transformed characters.

3. Loop through each character of the input string.

4. In each iteration, identify if the character is an uppercase letter, a lowercase letter, or a non-letter character.

5. If the character is an uppercase letter, perform the following operations:

   * Subtract the ASCII value of 'A' from the ASCII value of the character.
   * Add 13 to the result from the previous step.
   * Calculate the modulo of the result by 26, to wrap it around if it exceeds 'Z'.
   * Add the ASCII value of 'A' back to the result to get the final transformed character.

6. If the character is a lowercase letter, perform similar operations as step 5, but use 'a' instead of 'A' for the calculations to get the final transformed character.

7. If the character is not a letter, keep it unchanged.

8. Append the new character (transformed or unchanged) to the `result` string.

9. Once all characters have been processed, return the `result` string as the output of the function.

## Possible Attacks

The `ROT13` cipher is a `Caesar` cipher, a type of substitution cipher, which is quite simple and not meant for securing confidential information. Here are some possible attacks on it:

1. Brute-Force Attack:

      * Given its small key space (only 25 possible keys, excluding the identity transformation), an attacker can easily try all possible keys to decrypt the message.

2. Frequency Analysis:

      * Since ROT13 doesn't alter the frequency distribution of the characters, an attacker can perform frequency analysis to determine the most common letters and hence decipher the message.

3. Known-Plaintext Attack:

      * If an attacker has access to even a small piece of the plaintext and its corresponding ciphertext, they can easily determine the rotation used (which is 13 in the case of ROT13) and hence decrypt the entire message.

4. Ciphertext-Only Attack:

      * Given enough ciphertext, an attacker can potentially decrypt the message using statistical methods, as the ROT13 cipher doesn't provide diffusion.
