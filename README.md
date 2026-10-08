# SHA-256 - Educational Implementation

This project was developed for educational purposes. It should not be used in production.

An implementation of the SHA-256 hashing algorithm. This was developed to help my understanding of
the algorithm, and followed information presented in lectures.

A hash function takes an input message of arbitrary length and produces a fixed-length output, called a hash or digest. Hash functions are intended to be difficult to reverse and collision resistant. SHA-256 returns a 256-bit hash value.

## This Project Demonstrates
 - Hashing using the SHA-256 algorithm.

## Getting Started

### Prerequisites

- Rust 1.70 or later (with Cargo)

### Installation

```bash
git clone https://github.com/erikhurinek/sha256_impl
cd sha256_impl
cargo build --release
```

### Running the Demo

```bash
cargo run
```

This will:
1. Prompt you for a message to hash.
2. Output the hash of the message in hex format.

## Project Structure

```
sha256_impl/
├── Cargo.toml              # Project dependencies
├── README.md               # This file
└── src/
    ├── main.rs             # Demo application
    └── sha256.rs           # SHA-256 implementation
```

## Dependencies
```toml
hex = "0.4"
```

## Mathematical Method

Due to limitations in GitHub's Math rendering, some steps and questions may seem to be arranged in a strange order. This is unfortunately intentional as mathematical expressions cannot be indented, and nested lists cannot resume after expressions.

SHA-256 pads the message to a multiple of 512 bit, before processing each 512-bit block using 64 "rounds".  Each round updates 8 working variables of size 32-bits each. Any bitwise additions are done modulo $2^{32}$.

#### Initialise Helper Functions:

```math
\begin{align*}
 X \gg n   &\quad\; \text{Logical right shift $X$ by $n$ bits} \\
 X \ggg n  & \quad\; \text{Circular right shift $X$ by $n$ bits} \\
 \sigma_0(X,n) & = (X \ggg 7) \oplus (X \ggg 18) \oplus (X \gg 3) \\
 \sigma_1(X,n) & = (X \ggg 17) \oplus (X \ggg 19) \oplus (X \gg 10) \\
 \Sigma_0(X,n) & = (X \ggg 2) \oplus (X \ggg 13) \oplus (X \ggg 22) \\
 \Sigma_1(X,n) & = (X \ggg 6) \oplus (X \ggg 11) \oplus (X \ggg 25) \\
 Ch(X,Y,Z)    & = (X \land Y) \oplus (\overline{X} \land Z) \\
 Maj(X,Y,Z)   & = (X \land Y) \oplus (X \land Z) \oplus (Y \land Z)
\end{align*}
```

#### Initialise Variables and Constants:

1. Initialise the first hash values $H_0, H_1, \ldots, H_7$ to the following constants, which are the first 32 bits of the fractional parts of the square roots of the first 8 primes:

```math
\begin{align*}
   H_0 & = \texttt{0x6a09e667} \\
   H_1 & = \texttt{0xbb67ae85} \\
   &\ldots \\
   H_7 & = \texttt{0x5be0cd19}
\end{align*}
```

2. Initialise the round constants $K_0, K_1, \ldots, K_{63}$ to the following constants, which are the first 32 bits of the fractional parts of the cube roots of the first 64 primes:

```math
\begin{align*}
   K_0 & = \texttt{0x428a2f98} \\
   K_1 & = \texttt{0x71374491} \\
   &\ldots \\
   K_{63} & = \texttt{0xc67178f2}
\end{align*}
```

#### Process the Message:

1. Pad the message of length $L$:
    1. Append a bit 1 to the message.
    2. Pad the message with $k$ zeros such that when padded, the new message will be 64 bits less than a multiple of 512.
    3. Append the 64-bit representation of $L$ to the message.

```math
k = 512 - (L + 65) \bmod 512
```
2. Parse the padded message into $N$ 512-bit blocks.
3. For each block, perform the following:
    1. Prepare the message words $W_0, W_1, \ldots, W_{63}$:
    2. The first 16 words are obtained by splitting the block into 16 32-bit words. This is shown in the equation.
    3. For $i$ from 16 to 63, compute:
    4. Initialise the working variables $a, b, c, d, e, f, g, h$ to the current hash values $H_0, H_1, \ldots, H_7$.

```math
W_i = \sigma_1(W_{i-2}) + W_{i-7} + \sigma_0(W_{i-15}) + W_{i-16}
```

4. Still for each block, perform 64 rounds of the following:

```math
\begin{align*}
T_1 & = h + \Sigma_1(e) + Ch(e, f, g) + K_i + W_i \\
T_2 & = \Sigma_0(a) + Maj(a, b, c)
\end{align*}
```

```math
\begin{align*}
h & = g \\
g & = f \\
f & = e \\
e & = d + T_1 \\
d & = c \\
c & = b \\
b & = a \\
a & = T_1 + T_2
\end{align*}
```

5. After performing 64 rounds, update the hash values:

```math
\begin{align*}
H_0 & = H_0 + a \\
H_1 & = H_1 + b \\
&\ldots \\
H_7 & = H_7 + h
\end{align*}
```

4. The final hash is the concatenation of $H_0, H_1, \ldots, H_7$.
