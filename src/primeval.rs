// const CACHESIZE: u64 = (1 << 14) * 8; // this should be the size of the CPU L1 cache
use std::*;
use std::iter;


pub fn is_prime(n: usize) -> bool {
    if n > 1 {
        let primes = &primes_gen(n.clone()).collect::<Vec<_>>();
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
pub fn primes_gen(limit: usize) -> Box<Iterator<Item = usize>> {
    if limit < 3 {
        return if limit < 2 { Box::new(iter::empty::<usize>()) } else { Box::new(iter::once(2)) }
    }
 
    let indexlimit = (limit - 3) / 2 + 1;
    let buffersize = ((limit - 3) / 2) / 32 + 1;
    let mut composites = vec![0u32; buffersize];
    let sqrtindexlimit = ((limit as f64).sqrt() as usize - 3) / 2 + 1;
 
    for index in 0..sqrtindexlimit {
        if (composites[index >> 5] & (1u32 << (index & 31))) == 0 {
            let p = index + index + 3;
            let mut cullpos = (p * p - 3) / 2;
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
                if i < 0 { Some(2) } else {
                    if composites[i as usize >> 5] & (1u32 << (i & 31)) == 0 {
                        Some((i + i + 3) as usize) } else { None } }
    }))
}
