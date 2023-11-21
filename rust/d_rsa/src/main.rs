use std::io::{stdin, Read};
//use std::io::Read;

fn main() {
    let mut input = Vec::new();
    for byte in stdin().bytes(){
        input.extend(byte);
    }
    println!("{:?}", input);

    // TODO:
    //  - Choose two large prime numbers p and q
    //      -   https://docs.rs/is_prime/latest/is_prime/
    //      -   https://docs.rs/num-bigint/latest/num_bigint/
    //      -   https://medium.com/snips-ai/prime-number-generation-2a02f28508ff
    //  - Compute n = pq
    //  - Compute lambda(n), where lambda is Carmichael's totient function.
    //  - Choose an integer e such that 2 < e < lambda(n) and gcd(e, lambda(n)) = 1, e and lambda(n) are coprime
    //  - Determine d as d = e⁻¹(mod lambda(n))
}
