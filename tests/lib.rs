#[cfg(test)]
mod tests {
    use tck_no::tckn::{generate, validate};

    #[test]
    fn it_should_generate_tckn() {
        let result = generate();
        assert!(validate(&result));
    }
}
