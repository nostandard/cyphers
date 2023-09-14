# Polybius Square Cipher

The Polybius Square cipher, also known as the Polybius Checkerboard, is a form of substitution cipher. Here's how it works:

1. A 5x5 grid is created, filled with letters of the alphabet (the letter 'J' is usually omitted or combined with 'I').
2. Each letter in the grid is identified by its coordinates (row and column) in the grid.
3. To encrypt a message, replace each letter in the plaintext with the coordinates of that letter in the grid.
4. To decrypt a message, replace each pair of coordinates in the ciphertext with the letter found at that location in the grid.

Here is an example grid:

```bash
  1 2 3 4 5
1 A B C D E
2 F G H I K
3 L M N O P
4 Q R S T U
5 V W X Y Z
```

## Algorithm

1. ***Create a Grid with Letters***
    * Create a 5x5 grid which will act as the Polybius square.
    * Fill the grid with letters of the alphabet, excluding 'J' or combining it with 'I'.

2. ***Handle the Input***
    * Create a function that will accept a string as input. This string will be the message the user wants to encrypt or decrypt.
    * Create another function to clean the input string by removing any characters that are not part of the grid.

3. ***Encrypt the Message***
    * Create a function to encrypt a plaintext message.
    * In this function, iterate over each character in the cleaned input string.
    * For each character, find its coordinates in the grid.
    * Construct the ciphertext by concatenating the coordinates of each character: represent each character in the ciphertext as a string of two digits, where the first digit is the row number and the second digit is the column number.

4. ***Decrypt the Message***
    * Create a function to decrypt a ciphertext.
    * In this function, iterate over the ciphertext two characters at a time (since each pair of characters represents the coordinates of a letter in the grid).
    * Convert each pair of characters to their numerical values and use them as coordinates to find the corresponding letter in the grid.
    * Construct the plaintext by concatenating the letters found in the grid based on the coordinates.
