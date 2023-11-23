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
//  $ hexdump -vn256 -e'"%08X"' /dev/urandom | d_rsa
//
// or with openssl like:
//
//  $ openssl rand -hex 256 | d_rsa
//
// This is necessary as d_rsa only accepts hex string as input.

use hex;
use std::io;

fn turn_prime(number: &mut Vec<u8>) {
    // Turn prime
    //  1. LSB to 1
    //  2. Add 2 until it passes primality tests
}

fn main() {
    let mut input = String::new();
    let n = io::stdin().read_line(&mut input).unwrap();

    // Cast hex string into array of bytes
    let random_bytes = hex::decode(input).unwrap();

    // Split the array in half for p and q variables
    let mut p = &random_bytes[0..random_bytes.len() / 2];
    let mut q = &random_bytes[random_bytes.len() / 2..];

    // TODO:
    //  - Read two a large prime number from the random_generator and find two prime number from it,
    //      this can be accomplished by simply reading 256 bytes and using half of that for each
    //      variable p and q.
    //      -   https://docs.rs/is_prime/latest/is_prime/
    //      -   https://docs.rs/num-bigint/latest/num_bigint/
    //      -   https://medium.com/snips-ai/prime-number-generation-2a02f28508ff
    //  - Compute n = pq
    //  - Compute lambda(n), where lambda is Carmichael's totient function.
    //  - Choose an integer e such that 2 < e < lambda(n) and gcd(e, lambda(n)) = 1, e and lambda(n) are coprime
    //  - Determine d as d = e⁻¹(mod lambda(n))
}
