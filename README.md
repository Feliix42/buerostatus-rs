# buerostatus-rs

A small Rust crate that wraps the API of [fsr/buerostatus](https://github.com/fsr/buerostatus) and tells wou whether someone is in the office.

[![Build Status](https://travis-ci.org/Feliix42/buerostatus-rs.svg?branch=master)](https://travis-ci.org/Feliix42/buerostatus-rs)
[![Crates.io](https://img.shields.io/crates/v/buerostatus.svg)](https://crates.io/crates/buerostatus)

## Example
```rust
if let Ok(is_open) = get_buerostatus() {
    if is_open { println!("Someone's inside!"); }
    else { println!("No one is there..."); }
}
else {
    println!("An error occured!");
}
```
## Usage & Documentation
Please check the [documentation](https://feliix42.github.io/buerostatus-rs/) for details on certain functions and errors.

## Roadmap

- [x] Write some examples
- [x] Add travis
- [x] Documentation
- [ ] Maybe add function to get exact light value

## License

This work is licensed under the MIT License. For more information, head over to the `LICENSE` file.
