<h1 align="center">tck_no</h1>
<div align="center">
  <strong>
    Turkish Citizenship ID Validator and Generator Library for Rust
  </strong>
</div>

<br />

<div align="center">
  <a href="https://crates.io/crates/tck_no">
    <img src="https://img.shields.io/crates/v/tck_no.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <a href="https://crates.io/crates/tck_no">
    <img src="https://img.shields.io/crates/d/tck_no.svg?style=flat-square"
      alt="Download" />
  </a>
  <a href="https://docs.rs/tck_no">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/tck_no">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/peacecwz/tck_no/releases">
      Releases
    </a>
  </h3>
</div>

## Installation

With [cargo add][cargo-add] installed run:

```sh
$ cargo add tck_no
```

[cargo-add]: https://github.com/killercup/cargo-edit

## Generate TC Identity

```rs

use tck_no::tckn;

fn main() {
    let tc_identity = tckn::generate();
    println!("{}", tc_identity);
}

```

## Validate TC Identity

```rs

use tck_no::tckn;

fn main() {
    let tc_identity = "38246970008";

    if tckn::validate(tc_identity) {
        println!("Valid!");
    } else {
        println!("Invalid!");
    }
}

```

## Contributing

Want to join us? Look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

[good-first-issue]: https://github.com/peacecwz/tck_no/labels/good%20first%20issue
[help-wanted]: https://github.com/peacecwz/tck_no/labels/help%20wanted

## License

<sup>
Licensed under either of <a href="LICENSE.md">MIT license</a>
</sup>

