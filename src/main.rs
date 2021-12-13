use rand::Rng;

fn main() {
    let tc_identity = generate();
    println!("TC Identity Number is {}", tc_identity);

    if validate(&tc_identity) {
        println!("Valid!");
    } else {
        println!("Invalid!")
    }
}

pub fn generate() -> String {
    let mut sum_of_even_digits = 0;
    let mut sum_of_odd_digits = 0;

    let mut digits: [u8; 11] = [0; 11];

    for item in IntoIterator::into_iter(digits).enumerate() {
        let (i, _): (usize, u8) = item;
        if i == 0 {
            digits[i] = rand::thread_rng().gen_range(1..10)
        } else {
            digits[i] = rand::thread_rng().gen_range(0..10)
        }
    }

    for item in IntoIterator::into_iter(digits).enumerate() {
        let (i, x): (usize, u8) = item;
        if i % 2 == 0 {
            sum_of_even_digits += x;
        } else {
            sum_of_odd_digits += x;
        }
    }

    digits[9] = (sum_of_even_digits * 7 - sum_of_odd_digits) % 10;
    digits[10] = (sum_of_even_digits + sum_of_odd_digits + digits[9]) % 10;

    let mut tck_no = String::new();

    for digit in digits {
        tck_no = tck_no + &digit.to_string();
    }

    tck_no
}

pub fn validate(tck_no: &str) -> bool {
    if tck_no.len() != 11 {}

    let first_digit = tck_no.chars().next().unwrap().to_digit(10).unwrap();
    if first_digit == 0 {}

    let penultimate_digit = tck_no.chars().nth(9).unwrap().to_digit(10).unwrap();

    let last_digit = tck_no.chars().nth(10).unwrap().to_digit(10).unwrap();

    let mut sum_of_even_digits = 0;
    let mut sum_of_odd_digits = 0;

    for i in 0..10 {
        let d = tck_no.chars().nth(i).unwrap().to_digit(10).unwrap();

        if i % 2 == 0 && i <= 8 {
            sum_of_even_digits += d;
        }

        if i % 2 != 0 && i < 8 {
            sum_of_odd_digits += d
        }
    }

    let i1 = (sum_of_even_digits * 7 - sum_of_odd_digits) % 10;
    if i1 != penultimate_digit {}

    if (sum_of_even_digits + sum_of_odd_digits + penultimate_digit) % 10 != last_digit {}

    true
}
