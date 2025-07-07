mod mersenne_prime;
#[macro_use]
mod prime;

use std::process::exit;

use crate::mersenne_prime::is_mersenne_prime;

fn main() {
    let number: u64 = 25873;
    // let number: u64 = 104729;
    // let number: u64 = 1299709;
    // let number: u64 = 15485863;
    // let number: u64 = 2147483647;

    if !is_prime!(number) {
        println!(
            "Exponent {number} is not prime. Lucas-Lehmer test is only valid for prime exponents."
        );
        exit(0);
    }

    println!("Testing whether 2^{number} - 1 is a Mersenne prime...\n");

    if is_mersenne_prime(number) {
        println!("✅ 2^{number} - 1 is a Mersenne prime!");
    } else {
        println!("❌ 2^{number} - 1 is NOT a Mersenne prime.");
    }
}
