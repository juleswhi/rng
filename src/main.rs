use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let num: u8 = rng.gen_range(0..101);

    println!("Your number is: \x1b[1m\x1b[38;5;3m{}\x1b[22m", num);
}
