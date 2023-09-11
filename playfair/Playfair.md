# Playfair Cipher

The Playfair cipher is a manual symmetric encryption technique and was the first literal digraph substitution cipher. The scheme was invented in 1854 by Charles Wheatstone, but bears the name of Lord Playfair who promoted the use of the cipher. The encryption is done using a keyword to generate a 5x5 matrix of alphabets.

## Algorithm

### Step 1: Preparing the Key Matrix

#### Choosing a Keyword**

* Select a keyword, which will be used to create the key matrix. This keyword should preferably be a word that is easy to remember.

#### Creating a Unique Character List

* Prepare a string that contains each letter from the keyword only once. This means if a letter appears more than once in the keyword, subsequent occurrences are removed.

#### Creating the Key Matrix

* Create a 5x5 matrix.
* Fill this matrix first with the unique letters from the keyword.
* Next, fill in the remaining cells with the other letters of the alphabet that are not in the keyword, in order. Note that the letter "J" is traditionally omitted and any occurrence of "J" in the plaintext will be replaced by "I".

### Step 2: Preparing the Text

#### Cleaning the Text

* Take the plaintext input and remove any characters that are not letters (like spaces or punctuation).
* Convert all the letters to uppercase for consistency.
* Replace any occurrence of "J" (if omitted in the matrix) with "I".

#### Creating Digraphs

* Divide the cleaned text into pairs of letters (digraphs).
* If a pair contains two identical letters, insert an "X" (or another filler character) between them to separate them.
* If the cleaned text has an odd number of characters, add an "X" (or another filler character) to the end to make the number of characters even.

### Step 3: Encryption

#### Finding Letter Positions

* For each digraph in the prepared text, find the positions of both letters in the key matrix. The position will be denoted as a pair of row and column numbers.

#### Encrypting Digraphs

* If both letters are in the same row, replace them with the letters immediately to their right, wrapping around to the beginning of the row if necessary.
* If both letters are in the same column, replace them with the letters immediately below them, wrapping around to the top of the column if necessary.
* If the letters are in different rows and columns, form a rectangle with them as opposite corners, and replace them with the letters on the other two corners of the rectangle, i.e., the first letter is replaced by the letter in its row and the column of the second letter, and vice versa.

### Step 4: Outputting the Result

#### Joining the Encrypted Digraphs

* Concatenate the encrypted digraphs to form the encrypted text.

#### Displaying the Encrypted Text

* Display the encrypted text as the output.

### Step 5: Decryption

#### Reversing the Encryption Steps

* To decrypt, reverse the encryption steps: for letters in the same row, take the letters to their left; for letters in the same column, take the letters above them; for letters forming a rectangle, the substitution remains the same.

#### Joining the Decrypted Digraphs

* Concatenate the decrypted digraphs to form the decrypted text.

#### Displaying the Decrypted Text

* Display the decrypted text as the output.
