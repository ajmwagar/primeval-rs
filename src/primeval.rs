//! Primeval-rs
//!
//!A monstrosity of a prime number generator. (It's dead simple)
//!
//!## Features

//!- ZERO Dependencies (will always be this way)
//!- CLI Interface
//!- Rust library (see [crates.io](https://crates.io/crates/primeval))
//!
//!## Usage
//!
//!### CLI
//!
//!- `primeval help`: displays a help menu.
//!- `primeval gen <limit>`: generates all the prime numbers up to a limit
//!- `primeval prime <number>`: determines whether a number is prime or not
//!- `primeval version`: shows version info
//!
//!### Rust Crate
//!
//!*main.rs*
//!```rust
//!extern crate primeval;
//!
//!// We use usize to make sure nothing breaks when cross compiling.
//!fn main(){
//!  // Primality?
//!  let result = primeval::is_prime(2);
//!
//!  // Generation, in this case all the primes from 0 - 1000
//!  let result: Vec<usize> = primeval::primes_gen(1000).collect::<Vec<_>>();
//!}
//!```
//!
//!## Installation (CLI)
//!
//!1. `git clone https://github.com/ajmwagar/primeval-rs`
//!2. `cd primeval-rs`
//!3. `cargo build --release`
//!4. `cd target/release`
//!5. `./primeval help`
//!6. Profit!
//!
//!You can also move the binary into `/usr/bin` or somewhere else in your `PATH` to use from anywhere.
//!
//!## Tests & Benchmarks
//!
//!- To run the test suite: `cargo test`
//!- Always looking for more! (Submit a pull request)
//!- To benchmark Primeval: `cargo bench`
//!- Benchmarks prime number generation up to 1000000
//!
//!## Roadmap
//!
//!- [x] Rust Module/API
//!- [ ] Cleaner UI/CLI
//!- [ ] More SPEED!
//!- [ ] Factorization
//!- [ ] Larger number support
//!- [ ] Heat death of the universe

use std::*;
use std::iter;


/// Returns whether a given number is prime
pub fn is_prime(n: usize) -> bool {
    if n > 1 {
        if n >=  8 {
            // Get list of primes
            let primes = &primes_gen(n.clone() + 1).collect::<Vec<_>>();

            // Check if n is prime
            for &p in primes {
                let q: usize = n / p as usize;
                if q < p as usize { return true };
                let r = n - q * p as usize;
                if r == 0 { return false };
            }

            panic!("too few primes")
        }
        else {
            let primes = vec![2, 3, 5, 7];

            return primes.contains(&n);
        }
    }
    else {
        return false;
    }
}

/// Generate primes up to a given limit using the Seive of Eratorthenes
/// Use Sieve_of_Eratosthenes for prime generation (https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes)
pub fn primes_gen(limit: usize) -> Box<Iterator<Item = usize>> {
    if limit >= 10 {


    // Automatically return the small primes
    if limit < 3 {
        return if limit < 2 { Box::new(iter::empty::<usize>()) } else { Box::new(iter::once(2)) }
    }

    // Setup the indexs
    let indexlimit = (limit - 3) / 2 + 1;
    let buffersize = ((limit - 3) / 2) / 32 + 1;
    let mut composites = vec![0u32; buffersize];
    let sqrtindexlimit = ((limit as f64).sqrt() as usize - 3) / 2 + 1;

    // Go through the 0-sqrtindexlimit
    for index in 0..sqrtindexlimit {
        if (composites[index >> 5] & (1u32 << (index & 31))) == 0 {
            let p = index + index + 3;
            let mut cullpos = (p * p - 3) / 2;
            // Cull through the composites
            while cullpos < indexlimit {
                unsafe {
                    // avoids array bounds check, which is already done above
                    let cptr = composites.get_unchecked_mut(cullpos >> 5);
                    *cptr |= 1u32 << (cullpos & 31);
                }
                cullpos += p;
            }
        }
    }

    Box::new((-1 .. indexlimit as isize).into_iter().filter_map(move |i| {
        if i < 0 { 
            // Set initial prime
            Some(2)
        } 
        else {
            if composites[i as usize >> 5] & (1u32 << (i & 31)) == 0 {
                // Add this value to the box
                Some((i + i + 3) as usize) 
            } else { 
                // Dont add this value
                None 
            }
        }
    }))
    }
    else {
        return Box::new(vec![2,3,5,7].into_iter().filter(move |x| x <= &limit));
    }
}
