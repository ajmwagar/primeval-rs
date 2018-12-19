use std::env;


fn is_prime(n: usize, primes: &Vec<usize>) -> bool {
    for &p in primes {
        let q = n / p;
        if q < p { return true };
        let r = n - q * p;
        if r == 0 { return false };
    }
    panic!("too few primes")
}

// Use Sieve_of_Eratosthenes for prime generation (https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes)
fn primes_lt(bound: usize) -> Vec<usize> {
    let mut primes: Vec<bool> = (0..bound + 1).map(|num| num == 2 || num & 1 != 0).collect();
    let mut num = 3usize;
    while num * num <= bound {        
        let mut j = num * num;
        while j <= bound {
            primes[j] = false;
            j += num;
        }
        num += 2;
    }
    primes.into_iter().enumerate().skip(2).filter_map(|(i, p)| if p {Some(i)} else {None}).collect::<Vec<usize>>()
}

fn main(){
    let args: Vec<String> = env::args().collect();
    let sub = &args[1];
    if sub == "gen"{
        let value = &args[2].parse::<usize>().unwrap().clone();

        println!("{:?}",  primes_lt(*value));

    }
    else if sub == "prime"{
        let value = &args[2].parse::<usize>().unwrap().clone();

        let vec = primes_lt(*value);

        let res = is_prime(*value, &vec);
        println!("{}", res);

    }
    else if sub == "version"{
        let version = "Primeval 0.0.1 Copyright 2018 Avery Wagar";
        println!("{}", version);
    }
    else if sub == "help" {
        let help = "gen <upper bound>: generate primes until upper bound\nprime <number>: returns if number is prime\nhelp: shows this help menu\nversion: shows version info";
        println!("{}", help);
    }
    else {
        println!("Please issue a proper subcommand");
    }

}
