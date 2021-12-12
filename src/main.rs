use std::ffi::c_void;
use std::io;

fn main() {
    let tcIdentity = generate();
    println!("TC Identity Number is {}", tcIdentity);

    match validate(&tcIdentity) {
        Ok(_) => println!("Valid!"),
        Err(_) => println!("Invalid!")
    };
}

fn generate() -> String {
    let mut str: String = String::new();

    // TODO (peacecwz): Implement generation logic

    return str;
}

fn validate(&tck_no: &String) -> Result<String, E> {

    // TODO (peacecwz): Implement validation logic

    return Ok(tck_no);
}