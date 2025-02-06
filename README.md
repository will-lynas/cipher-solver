# Cipher Solver

A Rust library that provides tools for encrypting and decrypting various types of ciphers, as well as automatic solving using statistical analysis.


## Caesar Cipher

```rust
use cipher_solver::caesar;

// Encrypt a message
let message = "The quick brown fox jumps over the lazy dog";
let encrypted = caesar::encrypt(message, 3);

// Decrypt a message with known shift
let decrypted = caesar::decrypt(&encrypted, 3);

// Automatically solve a Caesar cipher with statistical analysis
let solved = caesar::solve(&encrypted);
```

### How it works

The solver uses statistical analysis to break Caesar ciphers without knowing the shift key. It:

1. Calculates letter frequencies in the encrypted text
2. Compares these frequencies with standard English letter frequencies using a chi-squared test
3. Tries all possible shifts (0-25) and selects the one that produces the most English-like text