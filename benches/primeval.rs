#![feature(test)]
extern crate test;
extern crate primeval;


#[cfg(test)]
mod tests {
    use primeval::*;
    use test::Bencher;

    #[bench]
    fn empty(b: &mut Bencher) {
        b.iter(|| 1)
    }

    #[bench]
    fn primeval_100_primes(b: &mut Bencher){
        b.iter(|| primeval::primes_gen(100 as usize));
    }
    #[bench]
    fn primeval_1k_primes(b: &mut Bencher){
        b.iter(|| primeval::primes_gen(1000 as usize));
    }
    #[bench]
    fn primeval_10k_primes(b: &mut Bencher){
        b.iter(|| primeval::primes_gen(10000 as usize));
    }
    #[bench]
    fn primeval_100k_primes(b: &mut Bencher){
        b.iter(|| primeval::primes_gen(100000 as usize));
    }
    #[bench]
    fn primeval_1m_primes(b: &mut Bencher){
        b.iter(|| primeval::primes_gen(1000000 as usize));
    }
    #[bench]
    fn primeval_10m_primes(b: &mut Bencher){
        b.iter(|| primeval::primes_gen(10000000 as usize));
    }
    // #[bench]
    // fn primeval_1b_primes(b: &mut Bencher){
    //     b.iter(|| primeval::primes_gen(1000000000 as usize));
    // }
    // #[bench]
    // fn primeval_10b_primes(b: &mut Bencher){
    //     b.iter(|| primeval::primes_gen(10000000000 as usize));
    // }
    // #[bench]
    // fn primeval_20b_primes(b: &mut Bencher){
    //     b.iter(|| primeval::primes_gen(20000000000 as usize));
    // }

    fn normalPrimeGen(n: usize) -> Vec<usize>{
        if n > 2 {
            let mut primes = Vec::new();
            primes.push(2);
            primes.push(3);
            primes.push(5);
            primes.push(7);
            primes.push(11);

            let mut res = true;
            for x in 0..n {
                if x % 2 != 0{

                    for y in 0..primes.len() {
                        if x % primes[y] == 0 {
                            res = false;
                        }
                    }
                    if res == true {
                        primes.push(x);
                    }
                    else{
                        res == false;
                    }
                }
            }

            return primes;
        }
        else {
            return Vec::new();
        }


    }

    #[bench]
    fn normal_100_primes(b: &mut Bencher){
        b.iter(|| normalPrimeGen(100 as usize));
    }
    #[bench]
    fn normal_1k_primes(b: &mut Bencher){
        b.iter(|| normalPrimeGen(1000 as usize));
    }
    #[bench]
    fn normal_10k_primes(b: &mut Bencher){
        b.iter(|| normalPrimeGen(10000 as usize));
    }
    #[bench]
    fn normal_100k_primes(b: &mut Bencher){
        b.iter(|| normalPrimeGen(100000 as usize));
    }
    // #[bench]
    // fn normal_1m_primes(b: &mut Bencher){
    //     b.iter(|| normalPrimeGen(1000000 as usize));
    // }
    // #[bench]
    // fn normal_10000000_primes(b: &mut Bencher){
    //     b.iter(|| normalPrimeGen(10000000 as usize));
    // }
}
