use argon2::Argon2;
use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::ChaCha20;
use sha2::{Digest, Sha256};

fn byte_generator(seed: &[u8], confusion_pattern: &[u8]) -> Vec<u8> {
    let nonce = [0x00; 12];

    let mut cipher = ChaCha20::new(seed.into(), &nonce.into());
    let mut buffer = seed.to_vec();

    let mut res = Vec::new();
    let mut found = false;

    loop {
        cipher.apply_keystream(&mut buffer);
        res.extend(buffer.clone());
        
        // TODO:
        //  - Check existence of confusion pattern and break if found
        //  - Whatever is there beyond the pattern is the next seed 
    }

    buffer
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

fn rand_byte_gen(password: &str, confusion_string: &str, _rounds: u32, length: usize) {
    let mut sha256 = Sha256::new();
    sha256.update(confusion_string);

    // Adjust the size of the seed array based on the desired length
    let mut seed = vec![0_u8; length];
    let _ = Argon2::default().hash_password_into(
        password.as_bytes(),
        &sha256.clone().finalize(),
        &mut seed,
    );
    sha256.reset();
    println!("Seed: {:02X?}", seed);

    let confusion_pattern = create_confusion_pattern(confusion_string);
    println!("Confusion Pattern: {:02X?}", confusion_pattern);

    let buffer_cipher = byte_generator(&seed, &confusion_pattern);
    println!("Buffer cipher: {:02X?}", buffer_cipher);
}

fn main() {
    rand_byte_gen("password1", "as", 10000, 32);
}
