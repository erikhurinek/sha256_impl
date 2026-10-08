// ---------------- Constants ----------------

// These are the first 32 bits of the fractional parts of the square roots of the first 8 primes.
const INITIAL_H: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

// These are the first 32 bits of the fractional parts of the cube roots of the first 64 primes.
const INITIAL_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

// ---------------- Helper Functions ----------------

fn sigma0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

fn sigma1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}

fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

fn big_sigma0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

fn big_sigma1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

// ---------------- Main Hash Function ----------------

pub fn hash(message: &[u8]) -> Vec<u8> {
    // Pad the message to a multiple of 512 bits (64 bytes).
    // The end of the message should contain "1" followed by the length in bits as a 64-bit
    // integer. Since this SHA-256 implementation operates on bytes, I simplified this
    // process to only pad whole bytes, not bits.
    let mut padded = message.to_vec();
    let bytes_to_pad = 64 - ((message.len() + 9) % 64);
    let bit_len = (message.len() as u64) * 8;
    padded.push(0x80);
    padded.extend(vec![0; bytes_to_pad]);
    padded.extend(&bit_len.to_be_bytes());

    // Process each 512-bit (64-byte) chunk
    let mut h = INITIAL_H;
    for chunk in padded.chunks(64) {
        // Generate 64 words from the chunk
        let mut w = [0u32; 64];
        // The first 16 words are directly from the chunk
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        // The remaining words are generated using a specific formula
        for i in 16..64 {
            w[i] = sigma1(w[i - 2])
                .wrapping_add(w[i - 7])
                .wrapping_add(sigma0(w[i - 15]))
                .wrapping_add(w[i - 16]);
        }

        // Perform the compression function

        // Set working variables to current hash value
        let mut v = h;
        // Perform 64 rounds
        for round in 0..64 {
            // Compute temp values
            let temp1 = v[7]
                .wrapping_add(big_sigma1(v[4]))
                .wrapping_add(ch(v[4], v[5], v[6]))
                .wrapping_add(INITIAL_K[round])
                .wrapping_add(w[round]);
            let temp2 = big_sigma0(v[0]).wrapping_add(maj(v[0], v[1], v[2]));

            // Shuffle the working variables
            v[7] = v[6];
            v[6] = v[5];
            v[5] = v[4];
            v[4] = v[3].wrapping_add(temp1);
            v[3] = v[2];
            v[2] = v[1];
            v[1] = v[0];
            v[0] = temp1.wrapping_add(temp2);
        }

        // Add the compressed chunk to the current hash value
        for i in 0..8 {
            h[i] = h[i].wrapping_add(v[i]);
        }
    }

    // Return the final hash value
    let mut digest = Vec::with_capacity(32);
    for &word in &h {
        digest.extend(&word.to_be_bytes());
    }

    digest
}
