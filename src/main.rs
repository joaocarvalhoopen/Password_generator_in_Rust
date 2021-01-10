//! # password_gen
//!
//! `password_gen` is a console program to generate a random password. 
//! The password can have n characters of length, and total percentage of 1.0 that
//! is compose from lower_case_perc, upper_case_perc, numbers_perc signs_perc.
//! 
//! ## Info
//! Author:  Joao Nuno Carvalho <br>
//! Date:    2021.01.10         <br>
//! License: MIT Open Source License <br>
//! 
//! ## References
//!
//! ## To compile the final program inside cargo
//! cargo build --release
//! 
//! ## To execute the program directly do
//! Usage: password_gen length <br>
//! Usage: password_gen length lower_case_% upper_case_% numbers_% signs_% <br>
//! <br>
//! ex: password_gen 8 <br>
//! ex: password_gen 8 0.35 0.35 0.15 0.15 <br>
//! 
//! ## To execute the program inside cargo
//! cargo run --release 8
//! cargo run --release 8 0.35 0.35 0.15 0.15
//! 
//! ## To generate the docs inside cargo
//! cargo doc <br>
//! cargo doc --open <br>


use std::env;
use std::process;
use rand::Rng;
use rand::rngs::ThreadRng;

const NUMBER_OF_LETTERS: usize = 26;
const NUMBER_OF_NUMBERS: usize = 10;
const NUMBER_SIGNS: usize      = 18;

/// Usage: "password_gen length"
static USAGE: &str = "   Usage: \"password_gen length\" \n          \"password_gen length lower_case_% upper_case_% numbers_% signs_%";

fn main() {
    println!("************************");
    println!("** PassWord Generator **");
    println!("************************");
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(& args);
    let mut pass_gen = PassGen::new(& cfg);
    pass_gen.generate(&cfg.pass_len);
    pass_gen.print();
    println!("...ended generating password.");
}

/// Configuration structure to parse the command line options.  
#[derive(Debug)]
#[derive(Copy, Clone)]
struct Config {
    pass_len: u32,

    lower_case_perc: f32,
    upper_case_perc: f32,
    numbers_perc: f32,
    signs_perc: f32,
}

impl Config {
    /// Constructor - Is were the parsing is made.
    /// It exists if an error occurs.
    fn new(args: &[String]) -> Config {
        if args.len() != 2 && args.len() != 6  {
            println!(" Invalid or insufficient parameters...");
            println!("{}", USAGE);
            process::exit(0)
        }
        let pass_len = match args[1].parse::<u32>(){
            Ok(n)  => n,
            Err(_e) => {
                println!(" Invalid password len, ex: password_gen 8  ...");
                println!("{}", USAGE);
                process::exit(0)
            } 
        };      
        let mut lower_case_perc = 0.35;
        let mut upper_case_perc = 0.35;
        let mut numbers_perc = 0.15;
        let mut signs_perc = 0.15;
        
        if args.len() > 2 {
            lower_case_perc = match args[2].parse::<f32>(){
                Ok(n)  => n,
                Err(_e) => {
                    println!(" Invalid lower_case_perc, ex: password_gen 8 0.35 0.35 0.15 0.15  ...");
                    println!("{}", USAGE);
                    process::exit(0)
                } 
            };
            upper_case_perc = match args[3].parse::<f32>(){
                Ok(n)  => n,
                Err(_e) => {
                    println!(" Invalid upper_case_perc, ex: password_gen 8 0.35 0.35 0.15 0.15  ...");
                    println!("{}", USAGE);
                    process::exit(0)
                } 
            };
            numbers_perc = match args[4].parse::<f32>(){
                Ok(n)  => n,
                Err(_e) => {
                    println!(" Invalid numbers_perc, ex: password_gen 8 0.35 0.35 0.15 0.15  ...");
                    println!("{}", USAGE);
                    process::exit(0)
                } 
            };
            signs_perc = match args[5].parse::<f32>(){
                Ok(n)  => n,
                Err(_e) => {
                    println!(" Invalid signs_perc, ex: password_gen 8 0.35 0.35 0.15 0.15  ...");
                    println!("{}", USAGE);
                    process::exit(0)
                } 
            };
            let total_perc = lower_case_perc + upper_case_perc + numbers_perc + signs_perc;
            if total_perc < 0.99 || total_perc > 1.01 {
                println!(" Invalid total values doesn't add to 1.0 ex: password_gen 8 0.35 0.35 0.15 0.15  ...");
                    println!("{}", USAGE);
                    process::exit(0)
            }
        }
        Config { pass_len, lower_case_perc, upper_case_perc, numbers_perc, signs_perc }
    }
}

/// The pass generating struct and the methods that fill the buffer.
#[derive(Debug)]
struct PassGen<'a> {
    cfg: & 'a Config,
    rng: ThreadRng,
    buf: String,
    letters_lower_case: [char; NUMBER_OF_LETTERS], 
    numbers: [char; NUMBER_OF_NUMBERS],
    signs: [char; NUMBER_SIGNS],
}

impl <'a> PassGen <'a> {
    /// Constructor
    fn new(cfg: & 'a Config) -> Self {
        PassGen {
            cfg: cfg, // cfg.clone(),
            rng: rand::thread_rng(),
            buf: String::from(""),
            letters_lower_case: ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'],
            signs: ['|','!','@','#','$','%','&','?','*','+','~','^',';',':','-','_','<','>'],
            numbers: ['0','1','2','3','4','5','6','7','8','9'],
        }
    }

    /// Fills the buffer with the password. 
    fn generate(& mut self, pass_len: & u32) {
        for _ in 0..*pass_len {
            // Generates a float between 0 and 1 not including 1.
            let letter_type_perc = self.rng.gen::<f32>();
            if letter_type_perc < self.cfg.lower_case_perc {
                let index = (self.rng.gen::<f32>() * self.letters_lower_case.len() as f32) as usize;
                self.buf.push(self.letters_lower_case[index]);
            } else if  letter_type_perc < self.cfg.lower_case_perc + self.cfg.upper_case_perc {
                let index = (self.rng.gen::<f32>() * self.letters_lower_case.len() as f32) as usize;
                let upper_case_array = self.letters_lower_case[index].to_uppercase().collect::<Vec<_>>(); 
                self.buf.push(upper_case_array[0]);
            } else if  letter_type_perc < self.cfg.lower_case_perc + self.cfg.upper_case_perc + self.cfg.numbers_perc {
                let index = (self.rng.gen::<f32>() * self.numbers.len() as f32) as usize;
                self.buf.push(self.numbers[index]);
            } else {
                let index = (self.rng.gen::<f32>() * self.signs.len() as f32) as usize;
                self.buf.push(self.signs[index]);
            }
        }
    }

    /// Prints the password. 
    fn print(& mut self) {
        println!("{}", self.buf);
    }

}

