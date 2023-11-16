use argon2::Argon2;
use sha2::{Digest, Sha256};

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
    // Adjust the size of the seed array based on the desired length
    let mut seed = vec![0_u8; length];
    let _ = Argon2::default().hash_password_into(
        password.as_bytes(),
        confusion_string.as_bytes(),
        &mut seed,
    );
    println!("Seed: {:02X?}", seed);

    let confusion_pattern = create_confusion_pattern(confusion_string);
    println!("Confusion Pattern: {:02X?}", confusion_pattern);

    // TODO:
    //  - What is the optimal length of the seed?
    //  - Confusion string length equal to what?
}

fn main() {
    rand_byte_gen("password", "asdaasda", 10000, 32);
}
