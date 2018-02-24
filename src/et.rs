use std::prelude::*;
extern crate failure;

use failure::{Error, err_msg};


pub type Result<T> = std::result::Result<T, Error>;

fn cr_err() -> Result<u32> {
    println!("about to error");
    //panic!("panicing here??");
    return Err(err_msg(format!("Some made up error {}:{}", file!(), line!())));
}

fn tier1() -> Result<u32> {
    let x = cr_err()?;
    Ok(x)
}

fn main() {
    println!("main");

    match tier1() {
        Ok(_) => println!("not error"),
        Err(err) => println!("some error = {}   trace: {:?}", err, err.backtrace()),
    }
}
