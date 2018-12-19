#![feature(test)]
extern crate test;

pub fn is_prime(n: usize) -> bool {
    if n > 1 {
        let primes = &primes_gen(n.clone());
        for &p in primes {
            let q: usize = n / p as usize;
            if q < p as usize { return true };
            let r = n - q * p as usize;
            if r == 0 { return false };
        }
        panic!("too few primes")
    }
    else {
        return false;
    }
}

// Use Sieve_of_Eratosthenes for prime generation (https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes)
pub fn primes_gen(bound: usize) -> Vec<usize> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn is_10_prime(){
        assert_eq!(false, is_prime(10 as usize));
    }
    #[test]
    fn is_5_prime(){
        assert_eq!(true, is_prime(5 as usize));
    }
    #[test]
    fn is_100_prime(){
        assert_eq!(false, is_prime(100 as usize));
    }
    #[test]
    fn is_two_prime_test(){
        assert_eq!(true, is_prime(2 as usize));
    }
    #[test]
    fn is_0_prime(){
        assert_eq!(false, is_prime(0 as usize));
    }
    #[test]
    fn is_1_prime(){
        assert_eq!(false, is_prime(1 as usize));
    }
    #[test]
    fn small_primes(){
        assert_eq!(primes_gen(20), vec![2,3,5,7,11,13,17,19]);
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
