macro_rules! is_prime {
    ($number:expr) => {{
        let number = $number;
        if number < 2 {
            false
        } else if number == 2 || number == 3 {
            true
        } else if number % 2 == 0 || number % 3 == 0 {
            false
        } else {
            let mut i: u64 = 5;
            let mut result = true;
            while i * i <= number {
                if number % i == 0 || number % (i + 2) == 0 {
                    result = false;
                    break;
                }
                i += 6;
            }
            result
        }
    }};
}
