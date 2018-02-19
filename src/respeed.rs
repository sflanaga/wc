extern crate regex;

use std::fmt;

use std::error;
use std::time::Instant;
use regex::Regex;
use std::env::args;

fn test_re_speed() -> fmt::Result {

    let iter = args().nth(1).expect("missing arg for iterations").parse::<usize>().expect("cannot parse iterations");
    let reStr = args().nth(2).expect("missing arg for re");
    let testStr : String = args().nth(3).expect("missing for test string");

    let start_f = Instant::now();

    //let re = Regex::new(r"^\D*(\d+)$").unwrap();
    let re = Regex::new(reStr.as_str()).unwrap();
    let mut tlen : usize = 0;
    for x in 0..iter {
        for cap in re.captures_iter(testStr.as_str()) {
            //println!("Num: {}", &cap[1]);
            tlen += &cap[1].len();
        }
    }
    let elapsed = start_f.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("secs: {}  {}", sec, tlen);

    Ok(())
}

fn main() {
    if let Err(e) = test_re_speed() {
        println!("error:{}  - usage: respeed re teststring", e);
        std::process::exit(1);
    }
}
