// Copyright 2023 David Araújo
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// *** ATTENTION ***
// To use d_rsa with other sources of random data, like /dev/urandom or OpenSSL,
// you must call it like:
//
// - /dev/urandom
//  $ hexdump -vn256 -e'"%08X"' /dev/urandom | d_rsa
//
// - OpenSSL:
//  $ openssl rand -hex 256 | d_rsa
//
// - random_generator:
//  $ random_generator password cf 100 32 256 | d_rsa
//
// This is necessary as d_rsa only accepts hex string as input!!.

use is_prime::*;
use std::io;
use num_bigint_dig::BigUint;
use num_traits::*;
use std::path::Path;
use base64ct::LineEnding;
use rsa::RsaPrivateKey;
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};

fn turn_prime(number: &mut Vec<u8>) -> BigUint {
    // Turn prime
    //  1. LSB to 1
    //  2. Add 2 until it passes primality tests

    // Bitwise OR turns the LSB to 1
    let len = number.len();
    number[len - 1] |= 0b00000001;

    // Turn vector into a Big Unsigned Number
    let mut big_number = BigUint::from_radix_be(number, 256).unwrap();

    // Increase the number until a prime number is found
    loop {
        // Uses the Miller-Rabin primality test algorithm
        if is_prime(&big_number.to_string()) {
            break;
        }
        big_number += 2u32;
    }

    big_number
}

fn main() {
    let mut received_stream = String::new();
    let _ = io::stdin().read_line(&mut received_stream).unwrap();

    // Cast hex string into array of bytes
    let random_bytes = hex::decode(received_stream.trim()).unwrap();

    // Split the array in half for p and q variables
    let (p, q) = random_bytes.split_at(random_bytes.len() / 2);

    // Turn into mutable vectors
    let mut p_vec = p.to_vec();
    let mut q_vec = q.to_vec();

    // Turn p and q into prime numbers
    let big_prime_p = turn_prime(&mut p_vec);
    let big_prime_q = turn_prime(&mut q_vec);

    // Calculate e
    let mut e = BigUint::parse_bytes(b"2", 10).unwrap();
    e = e.pow(16u32) + 1u32;
    
    let sk = RsaPrivateKey::from_p_q(big_prime_p, big_prime_q, e.clone()).unwrap();
    let pk = sk.to_public_key();

    // Verify validity
    if sk.validate().unwrap() == () {
        let sk_path = Path::new("./rust.pem");
        let pk_path = Path::new("./rust.pub");
        let _ = sk.write_pkcs8_pem_file(sk_path, LineEnding::default());
        let _ = pk.write_public_key_pem_file(pk_path, LineEnding::default());
    }
}
