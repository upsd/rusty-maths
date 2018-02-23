# Rusty Maths
A simple Rust library to perform basic arithmetic operations.

An example of how it can be used:
```rust
mod rusty_maths;

fn main() {
    println!(rusty_maths::add(1.0, 2.0)); // 3.0
    println!(rusty_maths::subtract(10.0, 2.0)); // 8.0
    println!(rusty_maths::multiply(10.0, 10.0)); // 100.0
    println!(rusty_maths::divide(21.0, 7.0)); // 3.0
}
```

To run the tests:
    
    $ cargo test

And to run the simple example program:

    $ cargo run