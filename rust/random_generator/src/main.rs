use std::env;

mod random_generator;

fn main() {
    let args: Vec<String> = env::args().collect();

    let password = &args[1];
    let confusion_string = &args[2];
    let rounds = args[3].parse::<u32>().unwrap();
    let byte_length = args[4].parse::<usize>().unwrap();
    let byte_amount = args[5].parse::<usize>().unwrap();

    let buffer = random_generator::rand_byte_gen(password, confusion_string, rounds, byte_length, byte_amount);

    // TESTING
    println!("{:?}", buffer);
}
