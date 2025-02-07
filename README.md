https://prod.liveshare.vsengsaas.visualstudio.com/join?1303A6135F03050416EF4CB1FAE2B6979CAB

# Asymmetric Cryptography Keymaker - Overview

This project implements Elliptic Curve Cryptography (ECC) in Rust to generate key pairs for file encryption and decryption.  It explores the mathematical concepts behind ECC and demonstrates their practical application using the Rust programming language.  This involved significant learning and implementation of cryptographic algorithms and a deeper understanding of the underlying mathematics.

## Description

The software generates asymmetric key pairs: a public key and a private key.  The public key can be shared and used to encrypt data, while the private key is kept secret and used to decrypt data. This ensures that only the holder of the private key can decrypt information encrypted with the corresponding public key.  The core of the key generation process involves elliptic curve operations, including point addition and point doubling, within a finite field defined by a modulus.  The private key is derived from the public key through these elliptic curve operations, forming a cyclic group of points.

## Purpose

The purpose of this software is to provide a practical demonstration of ECC key generation using Rust. It serves as a learning tool for understanding the principles of asymmetric cryptography and its implementation.  It can be used as a foundation for building more complex cryptographic applications, such as secure communication or data storage systems.

# Development Environment

## Tools

*   **Rust Compiler (rustc):** Used for compiling the Rust code.
*   **Cargo:** Rust's package manager, used for managing dependencies and building the project.
*   **Git:** Used for version control and collaboration.
*   **Charts:** Different Eliptical Curve Designs:
  ![Different Eliptical Curve Designs](https://github.com/Ambrosius1963/Asymmetric-Cryptography-keymaker/blob/11a8110adf47ffb42494c1b47e928a61325645ed/blob/Graphs.png?raw=true)

## Programming Language

*   **Rust:** The primary programming language used for development.

# Useful Websites

*   [Rust Programming Language](https://www.rust-lang.org/)
*   [Wikipedia: Elliptic-curve cryptography](https://en.wikipedia.org/wiki/Elliptic-curve_cryptography)
*   [Elliptic Curve Cryptography Tutorial](https://www.youtube.com/watch?v=gAtBM06xwaw)
*   [Elliptic Curve key pair generation](https://www.youtube.com/watch?v=wpLQZhqdPaA)

# Math Walk Through

Please see the Elliptic Curve Cryptography in Asymmetric Cryptography doc. for a walk through of the math.