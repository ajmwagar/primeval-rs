
extern crate primeval;

fn main(){
    use std::env;
    // Constants
    let version: String = "Primeval 0.0.2 Copyright 2018 Avery Wagar".to_string();
    let help: String = "Primeval: A montrosity of a prime generator.\n\nCommands\n=========================\ngen <upper bound>: generate primes until upper bound\nprime <number>: returns if number is prime\nhelp: shows this help menu\nversion: shows version info\n\nContact\n======================\ngithub: https://github.com/ajmwagar/primeval-rs\nemail: ajmw.subs@gmail.com".to_string();

    let args: Vec<String> = env::args().collect();
    if &args.len() > &1 {

        let sub = &args[1];
        let sub_slice: &str = &sub[..];

        match sub_slice {
            "gen" => println!("{:?}",  primeval::primes_gen(args[2].parse::<usize>().unwrap().clone())),
            "prime"=> println!("{}", primeval::is_prime(args[2].parse::<usize>().unwrap().clone())),
            "version" => println!("{}", version),
            "help" => println!("{}", help),
            _ => println!("Incorrect command. Try primeval help.")

        }
    }
    else {
        println!("No command issued. Try primeval help");
    }

}





