# Caesar Cipher

## Overview

The Caesar cipher is a type of substitution cipher where each letter in the plaintext is shifted a certain number of places down or up the alphabet. For instance, with a shift of 1, 'A' would be replaced by 'B', 'B' would become 'C', and so on. The method is named after Julius Caesar, who reportedly used it to communicate with his generals.

The formula to encrypt a letter using a Caesar cipher is:

`E<sub>n</sub>(x) = (x + n) mod 26`

Where:

* `E<sub>n</sub>(x)` is the encrypted letter
* `x` is the plaintext letter
* `n` is the shift value

The formula to decrypt is:

`D<sub>n</sub>(x) = (x - n) mod 26`

## Algorithm

1. ***User Input:*** Obtain the message to be encoded/decoded and the shift value.
2. ***Preprocess:*** Convert the message to uppercase (if you're only dealing with uppercase letters), and filter out any characters that aren't alphabetic.
3. ***Encryption:***

* For each letter in the message:
  * Calculate the encrypted character using the given formula.
  * Concatenate the result.

4. ***Decryption:***
Very similar to the encryption process, but you'll use the decryption formula instead.
5. ***Output:*** Display or return the result.
