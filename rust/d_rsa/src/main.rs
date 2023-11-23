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
// To use d_rsa with other sources of random data, like /dev/urandom,
// you can call it like:
//
// - /dev/urandom
//  $ hexdump -vn256 -e'"%08X"' /dev/urandom | d_rsa
//
// - Openssl:
//  $ openssl rand -hex 256 | d_rsa
//
// - random_generator:
//  $ random_generator password cs 100 32 256 | d_rsa
//
// This is necessary as d_rsa only accepts hex string as input.

use hex;
use is_prime::*;
use num_bigint::BigUint;
use num_traits::*;
use std::cmp::max;
use std::io;

// Function to calculate the greatest common divisor (GCD)
fn gcd(a: BigUint, b: BigUint) -> BigUint {
    if b.is_zero() {
        a
    } else {
        gcd(b.clone(), &a % &b)
    }
}

fn carmichael(p: BigUint, q: BigUint) -> BigUint {
    let phi_p = p.clone() - 1u32;
    let phi_q = q.clone() - 1u32;

    if p.is_zero() || q.is_zero() {
        BigUint::from_bytes_be(b"0")
    } else {
        // LCM(a, b) = |a * b| / GCD(a, b)
        let lcm_product = &p / gcd(phi_p.clone(), phi_q.clone()) * &q;
        max(lcm_product.clone(), lcm_product.clone()) // Ensure a positive result
    }
}

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
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input).unwrap();

    // Cast hex string into array of bytes
    let random_bytes = hex::decode(input.trim()).unwrap();

    // Split the array in half for p and q variables
    let (p, q) = random_bytes.split_at(random_bytes.len() / 2);

    // Turn into mutable vectors
    let mut p_vec = p.to_vec();
    let mut q_vec = q.to_vec();

    // Turn p and q into prime numbers
    let big_prime_p = turn_prime(&mut p_vec);
    let big_prime_q = turn_prime(&mut q_vec);

    // Calculate modulus
    let n = big_prime_p.clone() * big_prime_q.clone();

    let mut e = 2u32;
    e = e.pow(16) + 1;

    let lambda_n = carmichael(big_prime_p, big_prime_q);

    println!("{}", lambda_n);

    // TODO:
    //  - Private key
    //      - d
    // 
    //  - Public key
    //      - n
    //      - e
    //
    // READ:
    //  - https://medium.com/snips-ai/prime-number-generation-2a02f28508ff
}
