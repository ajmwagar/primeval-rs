# primeval-rs
[![Build Status](https://travis-ci.org/ajmwagar/primeval-rs.svg?branch=master)](https://travis-ci.org/ajmwagar/primeval-rs)

A monstrosity of a prime number generator. (It's dead simple)

## Features

- ZERO Dependencies (will always be this way)
- CLI Interface
- Rust library (see [crates.io](https://crates.io/crates/primeval))

## Usage

### CLI

- `primeval help`: displays a help menu.
- `primeval gen <limit>`: generates all the prime numbers up to a limit
- `primeval prime <number>`: determines whether a number is prime or not
- `primeval version`: shows version info

### Rust Crate

*main.rs*
```rust
extern crate primeval;

// We use usize to make sure nothing breaks when cross compiling.
fn main(){
  // Primality?
  let result = primeval::is_prime(&2 as usize);
  
  // Generation, in this case all the primes from 0 - 1000
  let result: Vec<usize> = primeval::primes_gen(&1000 as usize).collect::<Vec<_>>();
}
```

## Installation (CLI)

1. `git clone https://github.com/ajmwagar/primeval-rs`
2. `cd primeval-rs`
3. `cargo build --release`
4. `cd target/release`
5. `./primeval help`
6. Profit!

You can also move the binary into `/usr/bin` or somewhere else in your `PATH` to use from anywhere.

## Tests & Benchmarks

- To run the test suite: `cargo test`
  - Always looking for more! (Submit a pull request)
- To benchmark Primeval: `cargo bench`
  - Benchmarks prime number generation up to 1000000

## Roadmap

- [x] Rust Module/API
- [x] Cleaner UI/CLI
- [ ] More SPEED!
- [ ] Factorization
- [ ] Larger number support
- [ ] Heat death of the universe

