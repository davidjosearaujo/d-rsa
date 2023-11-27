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

use pbkdf2::pbkdf2_hmac;
use rc4::{consts::*, KeyInit, StreamCipher};
use rc4::{Key, Rc4};
use sha2::{Digest, Sha256};

fn contains<T: PartialEq>(haystack: &[T], needle: &[T]) -> bool {
    haystack.windows(needle.len()).any(|c| c == needle)
}

fn complex_seed_generator(seed: &[u8], confusion_pattern: &[u8], rounds: u32) -> Vec<u8> {
    let mut buffer = seed.to_vec();
    let mut seedx = seed.to_vec();

    for _n in 0..rounds {
        let key = Key::<U32>::from_slice(&seedx);
        let mut rc4 = Rc4::<_>::new(key);
        loop {
            rc4.apply_keystream(&mut buffer);
            if contains(&buffer, confusion_pattern) {
                seedx = buffer.clone();
                break;
            }
            
        }
        
    }

    seedx.to_vec()
}

pub fn rand_byte_gen(
    password: &str,
    confusion_string: &str,
    rounds: u32,
    seed_length: usize,
    byte_amount: usize,
) -> Vec<u8> {
    let mut sha256 = Sha256::new();
    sha256.update(confusion_string);

    // Adjust the size of the seed array based on the desired seed length
    let mut seed = vec![0_u8; seed_length];
    pbkdf2_hmac::<Sha256>(
        password.as_bytes(),
        &sha256.clone().finalize(),
        rounds,
        &mut seed,
    );

    // Create confusion pattern base on confusion string
    let confusion_pattern = &sha256.clone().finalize()[..confusion_string.len()];

    // Compute a complex seed
    let complex_seed = complex_seed_generator(&seed, &confusion_pattern, rounds);

    // Compute the requested number on random bytes using the complex seed
    let key = Key::<U32>::from_slice(&complex_seed);
    let mut rc4 = Rc4::<_>::new(key);
    let mut buffer = vec![0_u8; byte_amount];
    rc4.apply_keystream(&mut buffer);

    buffer
}