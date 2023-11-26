use std::env;
use std::process;
use hex;

mod random_generator;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 || &args[1] == "-h" {
        let help = "Random number generator\nUsage:\n\trandom_generator <password> <confusion-string> <rounds> <seed-length> <random-byte-amount>";
        println!("{}", help);
        process::exit(0x1);
    }

    let password = &args[1];
    let confusion_string = &args[2];
    let rounds = args[3].parse::<u32>().unwrap();
    let seed_length = args[4].parse::<usize>().unwrap();
    let byte_amount = args[5].parse::<usize>().unwrap();

    let buffer = random_generator::rand_byte_gen(
        password,
        confusion_string,
        rounds,
        seed_length,
        byte_amount,
    );

    print!("{}", hex::encode(buffer));
}
