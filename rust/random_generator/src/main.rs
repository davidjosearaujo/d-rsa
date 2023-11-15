use pbkdf2::pbkdf2_hmac_array;
use argon2::Argon2;
use sha2::Sha256;

fn rand_byte_gen(password: &str, confusion_string: &str, rounds: u32){
    
    // 256 Byte seed calculation using PBKDF2
    let seed = pbkdf2_hmac_array::<Sha256, 32>(
        password.as_bytes(),
        confusion_string.as_bytes(),
        rounds,
    );
    println!("Seed: {:02X?}", seed);

    // TODO: 
    //  - What is the optimal length of the seed?
    //  - Confusion string length equal to what?
}

fn main() {
    println!("Hello, world!");
    rand_byte_gen("password", "asdadeeqrqrqwreas", 10000);
}
