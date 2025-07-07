use std::process::exit;

use indicatif::{ProgressBar, ProgressStyle};
use rug::Integer;

fn is_prime(number: u64) -> bool {
    if number < 2 {
        return false;
    }

    if number == 2 || number == 3 {
        return true;
    }

    if number % 2 == 0 || number % 3 == 0 {
        return false;
    }

    let mut i: u64 = 5;
    while i * i <= number {
        if number % i == 0 || number % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn is_mersenne_prime(number: u64) -> bool {
    let mut sequence_value: Integer = Integer::from(4);
    let mut mersenne_candidate: Integer = Integer::from(1);
    mersenne_candidate <<= number as u32; // 2^number
    mersenne_candidate -= 1; // 2^number - 1

    let iterations: u64 = number - 2;
    let progress_bar: ProgressBar = ProgressBar::new(iterations);
    progress_bar.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("=> "),
    );

    for _ in 0..iterations {
        sequence_value.square_mut();
        sequence_value -= 2;
        sequence_value %= &mersenne_candidate;
        progress_bar.inc(1);
    }
    progress_bar.finish_with_message("Lucas-Lehmer test complete");

    sequence_value == 0
}

fn main() {
    let number: u64 = 25873;
    // let number: u64 = 104729;
    // let number: u64 = 1299709;
    // let number: u64 = 15485863;
    // let number: u64 = 2147483647;

    if !is_prime(number) {
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
