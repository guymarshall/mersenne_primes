use indicatif::{ProgressBar, ProgressStyle};
use rug::Integer;

pub(crate) fn is_mersenne_prime(number: u64) -> bool {
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
