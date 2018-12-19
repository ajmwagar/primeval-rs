#![feature(test)]
extern crate test;
extern crate primeval;


#[cfg(test)]
mod tests {
    use primeval::*;
    use test::Bencher;

    #[test]
    fn is_10_prime(){
        assert_eq!(false, primeval::is_prime(10 as usize));
    }
    #[test]
    fn is_5_prime(){
        assert_eq!(true, primeval::is_prime(5 as usize));
    }
    #[test]
    fn is_100_prime(){
        assert_eq!(false, primeval::is_prime(100 as usize));
    }
    #[test]
    fn is_two_prime_test(){
        assert_eq!(true, primeval::is_prime(2 as usize));
    }
    #[test]
    fn is_0_prime(){
        assert_eq!(false, primeval::is_prime(0 as usize));
    }
    #[test]
    fn is_1_prime(){
        assert_eq!(false, primeval::is_prime(1 as usize));
    }
    fn is_7_prime(){
        assert_eq!(true, primeval::is_prime(7 as usize));

    }
    #[test]
    fn small_primes(){
        assert_eq!(primeval::primes_gen(20), vec![2,3,5,7,11,13,17,19]);
    }

    #[bench]
    fn bench_100_primes(b: &mut Bencher){
        b.iter(|| primeval::primes_gen(100 as usize));
    }
    #[bench]
    fn bench_1000_primes(b: &mut Bencher){
        b.iter(|| primeval::primes_gen(1000 as usize));
    }
    #[bench]
    fn bench_10000_primes(b: &mut Bencher){
        b.iter(|| primeval::primes_gen(10000 as usize));
    }
    #[bench]
    fn bench_100000_primes(b: &mut Bencher){
        b.iter(|| primeval::primes_gen(100000 as usize));
    }
    #[bench]
    fn bench_1000000_primes(b: &mut Bencher){
        b.iter(|| primeval::primes_gen(1000000 as usize));
    }
}
