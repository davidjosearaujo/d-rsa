use std::io::{stdin, Read};
//use std::io::Read;

fn main() {
    let mut input = Vec::new();
    for byte in stdin().bytes(){
        input.extend(byte);
    }
    println!("{:?}", input);

    // TODO:
    //  - Create RSA key pair
}
