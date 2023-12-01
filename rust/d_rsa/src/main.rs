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
use num_bigint_dig::BigUint;
use num_bigint_dig::ModInverse;
use num_integer::Integer;
use num_traits::*;
use std::io;
use std::str::from_utf8;
use std::fs::File;
use std::io::Write;
use base64::{engine::general_purpose, Engine as _};


#[derive(Clone, PartialEq, Debug)]
pub struct KeyPair {
    pub pk: PublicKey,
    pub sk: SecretKey,
}

#[derive(Clone, PartialEq, Debug)]
pub struct PublicKey {
    pub n: BigUint,
    pub e: BigUint,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SecretKey {
    pub n: BigUint,
    pub d: BigUint,
}

impl KeyPair {
    pub fn new(_pk: &PublicKey, _sk: &SecretKey) -> Result<Self, &'static str> {
        let kp = KeyPair {
            pk: _pk.to_owned(),
            sk: _sk.to_owned(),
        };
        Ok(kp)
    }

    pub fn print(&self) {
        let mut pk_file = File::create("rs_rsa_pk.pub").unwrap();
        let mut sk_file = File::create("rs_rsa_sk.pem").unwrap();
        //Ask for encoded params and write.
        let (pk, sk) = prepare_to_print(&self);
        pk_file.write_all(pk.as_bytes()).unwrap();
        sk_file.write_all(sk.as_bytes()).unwrap();
    }
}

impl PublicKey {
    /// Generate a PublicKey struct from n and d co-prime factors.
    pub fn new(_n: &BigUint, _e: &BigUint) -> Result<Self, &'static str> {
        Ok(PublicKey {
            n: _n.to_owned(),
            e: _e.to_owned(),
        })
    }
}

impl SecretKey {
    /// Generate a SecretKey struct from n and d co-prime factors.
    pub fn new(_n: &BigUint, _d: &BigUint) -> Result<Self, &'static str> {
        Ok(SecretKey {
            n: _n.to_owned(),
            d: _d.to_owned()
        })
    }
}

pub fn prepare_to_print(kp: &KeyPair) -> (String, String) {
    let (mut encoded_pk, mut encoded_sk) = (String::new(), String::new());

    // Encoding Public Key
    encoded_pk.push_str("---------- BEGIN RSA PUBLIC KEY ----------");
    encoded_pk.push_str("\n");
    let mut binding = general_purpose::STANDARD.encode(&kp.pk.n.to_bytes_be());
    let mut priv_n = from_utf8(binding.as_bytes()).unwrap();
    for i in (64..priv_n.len()).step_by(64) {
        encoded_pk.push_str(&priv_n[i-64..i]);
        encoded_pk.push_str("\n");
    }

    binding = general_purpose::STANDARD.encode(&kp.pk.e.to_bytes_be());
    priv_n = from_utf8(binding.as_bytes()).unwrap();
    for i in (64..priv_n.len()).step_by(64) {
        encoded_pk.push_str(&priv_n[i-64..i]);
        encoded_pk.push_str("\n");
    } 
    encoded_pk.push_str("----------- END RSA PUBLIC KEY -----------");
    
    // Encoding Secret Key
    encoded_sk.push_str("---------- BEGIN RSA PRIVATE KEY ----------");
    encoded_sk.push_str("\n");
    binding = general_purpose::STANDARD.encode(&kp.sk.n.to_bytes_be());
    priv_n = from_utf8(binding.as_bytes()).unwrap();
    for i in (64..priv_n.len()).step_by(64) {
        encoded_sk.push_str(&priv_n[i-64..i]);
        encoded_sk.push_str("\n");
    }

    binding = general_purpose::STANDARD.encode(&kp.sk.d.to_bytes_be());
    priv_n = from_utf8(binding.as_bytes()).unwrap();
    for i in (64..priv_n.len()).step_by(64) {
        encoded_sk.push_str(&priv_n[i-64..i]);
        encoded_sk.push_str("\n");
    }
    encoded_sk.push_str("----------- END RSA PRIVATE KEY -----------");
    (encoded_pk, encoded_sk)
}

fn carmichael(p: BigUint, q: BigUint) -> BigUint {
    let phi_p = p.clone() - 1u32;
    let phi_q = q.clone() - 1u32;

    if phi_p.is_zero() || phi_q.is_zero() {
        BigUint::zero()
    } else {
        phi_p.lcm(&phi_q)
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

    // Calculate modulus
    let n = big_prime_p.clone() * big_prime_q.clone();

    // Calculate e
    let mut e = BigUint::parse_bytes(b"2", 10).unwrap();
    e = e.pow(16u32) + 1u32;

    // Carmichael's totient function
    let lambda_n = carmichael(big_prime_p, big_prime_q);

    // Inverse modulus of ƛ(n)
    let d = e.clone().mod_inverse(&lambda_n).unwrap();
    
    let pk = PublicKey::new(&n, &e).unwrap();
    let sk = SecretKey::new(&n, &d.to_biguint().unwrap()).unwrap();

    let kp = KeyPair::new(&pk, &sk).unwrap();
    kp.print();
}
