//! Author: Masahiro Itabashi <itabasi.lm@gmail.com>
//! Last modified: Mon, 11 Jan 2021 22:15:00 +0900
pub mod parse_args;
use parse_args::Args;

use std::time::Duration;
use rppal::pwm::{Channel, Polarity, Pwm};

fn main(){
    let args: Args = Args::parse_args();

    if args.type_f() {
        Pwm::with_frequency(
            Channel::Pwm0,
            args.flag_f.unwrap(),
            args.flag_c.unwrap(),
            Polarity::Normal,
            true
        );
    }
    else if args.type_p() {
        Pwm::with_period(
            Channel::Pwm0,
            Duration::from_secs_f64(args.flag_p.unwrap()),
            Duration::from_secs_f64(args.flag_w.unwrap()),
            Polarity::Normal,
            true
        );
    }
    else {
        println!("hoge");
    }
}

