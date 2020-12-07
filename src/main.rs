//! Author: Masahiro Itabashi <itabasi.lm@gmail.com>
//! Last modified: Tue, 08 Dec 2020 05:53:43 +0900
use std::env::args;

use rppal::pwm::{Channel, Polarity, Pwm};

fn print_usage(){
    println!("Usage: ./ctl_servomotor frequency duty_ratio");
}

//TODO
//Change to return tuple with Polarity
fn parse_args() -> (f64, f64) {
    let args: Vec<String> = args().collect();
    let mut freq: f64 = 40.0;
    let mut duty: f64 = 0.2;

    match args.len(){
        1 => {},
        2 => {
            freq = args[1].parse().unwrap();
        },
        3 => {
            freq = args[1].parse().unwrap();
            duty = args[2].parse().unwrap();
        },
        _ => {
            print_usage();
        }
    };
    (freq, duty)
}

fn main(){
    let args = parse_args();
    let res = Pwm::with_frequency(
        Channel::Pwm0,
        args.0,
        args.1,
        Polarity::Normal,
        true
        );

    match res{
        Ok(Pwm) => {
            println!("PWM executed");
        },
        Err(_) => {
            println!("Error: Can't open device");
        }
    };
}

