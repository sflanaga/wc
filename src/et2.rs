use std::prelude::*;

type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;

fn cr_err() -> GenResult<u32> {
    println!("about to error");
    //panic!("panicing here??");
    return Err(GenError::from(format!("Some made up error {}:{}", file!(), line!())));
}

fn tier1() -> GenResult<u32> {
    let x = cr_err()?;
    Ok(x)
}

fn main() {
    println!("main");

    match tier1() {
        Ok(_) => println!("not error"),
        Err(err) => println!("some error = {}", err),
    }
}
