use pbkdf2::pbkdf2_hmac;
use rc4::{consts::*, KeyInit, StreamCipher};
use rc4::{Key, Rc4};
use sha2::{Digest, Sha256};

fn is_sub<T: PartialEq>(haystack: &[T], needle: &[T]) -> bool {
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
            if is_sub(&buffer, confusion_pattern) {
                seedx = buffer.clone();
                break;
            }
        }
    }

    seedx.to_vec()
}

fn create_confusion_pattern(confusion_string: &str) -> Vec<u8> {
    let mut sha256 = Sha256::new();

    // Initialize a Vec<u8> to store the final confusion pattern
    let mut confusion_pattern = Vec::new();

    let chunk_size = 256;
    for chunk in confusion_string.as_bytes().chunks(chunk_size) {
        sha256.update(chunk);
        confusion_pattern.extend_from_slice(&sha256.clone().finalize());
        sha256.reset();
    }

    // Trim the confusion pattern to match the length of the input string
    confusion_pattern.truncate(confusion_string.len());

    confusion_pattern
}

fn rand_byte_gen(
    password: &str,
    confusion_string: &str,
    rounds: u32,
    seed_length: usize,
    _byte_amount: usize,
) {
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

    let confusion_pattern = create_confusion_pattern(confusion_string);

    let complex_seed = complex_seed_generator(&seed, &confusion_pattern, rounds);

    println!("Complex Seed: {:02X?}", complex_seed);
}

fn main() {
    rand_byte_gen("password", "fg", 10000, 32, 10);
}
