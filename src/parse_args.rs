//! Author: Masahiro Itabashi <itabasi.lm@gmail.com>
//! Last modified: Mon, 11 Jan 2021 22:10:58 +0900
use std::env;

use docopt::Docopt;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Args {
    pub flag_f: Option<f64>,
    pub flag_c: Option<f64>,
    pub flag_p: Option<f64>,
    pub flag_w: Option<f64>,
}

impl Args {
    pub fn parse_args() -> Args {
        const USAGE: &'static str = "
        Control a servo mortor with PWM
        
        Usage:
          ctl_servomotor
          ctl_servomotor -f FREQUENCY -c DUTYCYCLE
          ctl_servomotor -p PERIOD -w PULTHWIDTH
          ctl_servomotor (-h | --help)

        Options:
          -f FREQUENCY      specify frequency
          -c DUTYCYCLE      specify duty cycle
          -p PERIOD         specify period
          -w PULTHWIDTH     specify puth width
          -h --help         output this help
        ";

        let args: Args = Docopt::new(USAGE)
            .and_then(|d| d.argv(env::args().into_iter()).deserialize())
            .unwrap_or_else(|e| e.exit());
        args
    }
    
    pub fn type_f(&self) -> bool {
        if self.flag_f != None {
            true
        }
        else {
            false
        }
    }

    pub fn type_p(&self) -> bool {
        if self.flag_p != None {
            true
        }
        else {
            false
        }
    }
}
