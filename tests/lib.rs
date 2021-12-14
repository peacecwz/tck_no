#[cfg(test)]
mod tests {
    use tck_no::tckn::{generate, validate};

    #[test]
    fn it_should_generate_tck_no() {
        let tck_no = generate();

        println!("TC Identity Number: {}", tck_no);

        let result = validate(&tck_no);
        assert!(result);
    }
}
