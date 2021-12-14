pub mod tckn {
    use rand::Rng;

    pub fn generate() -> String {
        let mut sum_of_even_digits = 0;
        let mut sum_of_odd_digits = 0;

        let mut digits: [u8; 11] = [0; 11];
        digits[0] = rand::thread_rng().gen_range(1..10);

        for digit in digits.iter_mut().skip(1) {
            *digit = rand::thread_rng().gen_range(0..10);
        }

        for item in digits.iter().take(9).enumerate() {
            let (i, x) = item;

            if i % 2 == 0 {
                sum_of_even_digits += x;
            } else {
                sum_of_odd_digits += x;
            }
        }

        digits[9] = (sum_of_even_digits * 7 - sum_of_odd_digits) % 10;
        digits[10] = (sum_of_even_digits + sum_of_odd_digits + digits[9]) % 10;

        let mut tck_no = String::with_capacity(11);

        for digit in digits {
            tck_no.push_str(&digit.to_string());
        }

        tck_no
    }

    pub fn validate(tck_no: &str) -> bool {
        if tck_no.len() != 11 {
            return false;
        }

        let first_digit = tck_no.chars().next().unwrap().to_digit(10).unwrap();
        if first_digit == 0 {
            return false;
        }

        let penultimate_digit = tck_no.chars().nth(9).unwrap().to_digit(10).unwrap();

        let last_digit = tck_no.chars().nth(10).unwrap().to_digit(10).unwrap();

        let mut sum_of_even_digits = 0;
        let mut sum_of_odd_digits = 0;

        for i in 0..10 {
            let digit = tck_no.chars().nth(i).unwrap().to_digit(10).unwrap();

            if i % 2 == 0 && i <= 8 {
                sum_of_even_digits += digit;
            }

            if i % 2 != 0 && i < 8 {
                sum_of_odd_digits += digit;
            }
        }

        let i1 = (sum_of_even_digits * 7 - sum_of_odd_digits) % 10;
        if i1 != penultimate_digit {
            return false;
        }

        if (sum_of_even_digits + sum_of_odd_digits + penultimate_digit) % 10 != last_digit {
            return false;
        }

        true
    }
}
