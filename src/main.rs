#![feature(test)]
extern crate test;

use std::env;


fn is_prime(n: usize, primes: &Vec<usize>) -> bool {
    for &p in primes {
        let q: usize = n / p as usize;
        if q < p as usize { return true };
        let r = n - q * p as usize;
        if r == 0 { return false };
    }
    panic!("too few primes")
}

// Use Sieve_of_Eratosthenes for prime generation (https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes)
fn primes_gen(bound: usize) -> Vec<usize> {
    let mut primes: Vec<bool> = (0..bound + 1).map(|num| num == 2 || num & 1 != 0).collect();
    let mut num = 3usize;
    while num * num <= bound {        
        let mut j = num * num;
        while j <= bound {
            primes[j as usize] = false;
            j += num;
        }
        num += 2;
    }
    primes.into_iter().enumerate().skip(2).filter_map(|(i, p)| if p {Some(i)} else {None}).collect::<Vec<usize>>()
}

fn main(){
    // Constants
    let version: String = "Primeval 0.0.2 Copyright 2018 Avery Wagar".to_string();
    let help: String = "Primeval: A montrosity of a prime generator.\n\nCommands\n=========================\ngen <upper bound>: generate primes until upper bound\nprime <number>: returns if number is prime\nhelp: shows this help menu\nversion: shows version info\n\nContact\n======================\ngithub: https://github.com/ajmwagar/primeval-rs\nemail: ajmw.subs@gmail.com".to_string();

    let args: Vec<String> = env::args().collect();
    if &args.len() > &1 {

    let sub = &args[1];
    let sub_slice: &str = &sub[..];

    match sub_slice {
        "gen" => println!("{:?}",  primes_gen(args[2].parse::<usize>().unwrap().clone())),
        "prime"=> println!("{}", is_prime(args[2].parse::<usize>().unwrap().clone(), &primes_gen(args[2].parse::<usize>().unwrap().clone()))),
        "version" => println!("{}", version),
        "help" => println!("{}", help),
        _ => println!("Incorrect command. Try primeval help.")

    }
    }
    else {
        println!("No command issued. Try primeval help");
    }

}



#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn is_10_prime(){
        assert_eq!(false, is_prime(10 as usize, &primes_gen(10 as usize)));
    }
    #[test]
    fn is_5_prime(){
        assert_eq!(true, is_prime(5 as usize, &primes_gen(5 as usize)));
    }
    #[test]
    fn is_100_prime(){
        assert_eq!(false, is_prime(100 as usize, &primes_gen(100 as usize)));
    }
    #[test]
    fn is_two_prime_test(){
        assert_eq!(true, is_prime(2 as usize, &primes_gen(2 as usize)));
    }

    #[bench]
    fn bench_100_primes(b: &mut Bencher){
        b.iter(|| primes_gen(100 as usize));
    }
    #[bench]
    fn bench_1000_primes(b: &mut Bencher){
        b.iter(|| primes_gen(1000 as usize));
    }
    #[bench]
    fn bench_10000_primes(b: &mut Bencher){
        b.iter(|| primes_gen(10000 as usize));
    }
    #[bench]
    fn bench_100000_primes(b: &mut Bencher){
        b.iter(|| primes_gen(100000 as usize));
    }
    #[bench]
    fn bench_1000000_primes(b: &mut Bencher){
        b.iter(|| primes_gen(1000000 as usize));
    }
}
