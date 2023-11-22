use hex;
use std::io;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input).unwrap();

    // Remove trailing newline and encode to Vec<u8>
    input.pop();
    let random_bytes = hex::decode(input).unwrap();

    println!("{:02X?}", random_bytes);

    // TODO:
    //  - Read two a large prime number from the random_generator and find two prime number from it,
    //      this can be accomplished by simply reading 2048 bytes and using half of that for each
    //      variable p and q.
    //      -   https://docs.rs/is_prime/latest/is_prime/
    //      -   https://docs.rs/num-bigint/latest/num_bigint/
    //      -   https://medium.com/snips-ai/prime-number-generation-2a02f28508ff
    //  - Compute n = pq
    //  - Compute lambda(n), where lambda is Carmichael's totient function.
    //  - Choose an integer e such that 2 < e < lambda(n) and gcd(e, lambda(n)) = 1, e and lambda(n) are coprime
    //  - Determine d as d = e⁻¹(mod lambda(n))
}
