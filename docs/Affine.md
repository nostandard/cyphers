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
