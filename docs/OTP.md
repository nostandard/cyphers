# One-Time Pad (OTP)

## Overview

The One-Time Pad (OTP) cipher is a symmetric encryption technique that requires a key, called a pad, that is as long as the message to be encrypted. When used correctly, it provides perfect secrecy. The OTP works by combining each bit or character of the plaintext with the corresponding bit or character from the key using bitwise exclusive OR (XOR).

The main principles of the OTP are:

* The key is truly random.
* The key is at least as long as the message or data it encrypts.
* The key is used only once and then discarded.

Decryption is done by applying the same key to the ciphertext, using the XOR operation again.

## Algorithm

1. *Preparation*:

* Get the message you want to encrypt. This is called the plaintext.
* Decide on the length of the message (count how many characters or bits it has).

2. *Key Generation*:

* For every bit (or character) in the message, generate a random corresponding bit (or character). This process will give you a sequence that's as long as the message.
* This sequence is called the key, and it's crucial for both encryption and decryption.

3. *Encryption*:

* Go through the message bit by bit.
* For each bit in the message, take the corresponding bit from the key you generated.
* Use an operation called XOR (exclusive OR) on these bits. This operation takes two bits and gives you a new bit: if the original bits are different, the result is 1; if they're the same, the result is 0.
* The result of this XOR operation is the encrypted message, or the ciphertext.

4. *Sending/Storage*:

* You can now safely send or store this encrypted message (ciphertext). Even if someone intercepts or sees it, they can't understand or decrypt it without the key.

5. *Decryption*:

* When you want to turn the encrypted message (ciphertext) back into the original message, you'll need the key.
* Go through the ciphertext bit by bit.
* For each bit in the ciphertext, take the corresponding bit from the key.
* Use the XOR operation again on these bits. It's the same process as encryption, because XOR is its own inverse.
* The result is the original message, or plaintext.

6. *Key Destruction*:

* Once you've decrypted the message, the key should be destroyed and never used again.
* Remember, the strength of OTP lies in the randomness of the key and the fact that it's used only once. If you use the same key for a different message, the security is compromised.

## Problems
