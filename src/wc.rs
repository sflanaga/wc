#![allow(unused_imports)]

extern crate bytecount;
use std::borrow::Borrow;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Error;
use std::thread;
// use std::io::StdinLock;
// use std::io::StdinLock::read;
use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;

pub fn mainfile<'a, I>(filenames: I, _slow: bool)
where
    I: Iterator<Item = &'a str>,
{

    let mut buf: [u8; 1024 * 128] = [0; 1024 * 128];

    for arg in filenames {
        let a = arg;
        // let a = arg.borrow();
        let path = Path::new(&a);
        let mut f = match OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(&path)
            {
                Ok(f) => f,
                Err(e) => panic!("cannot open file \"{}\" due to this error: {}", path.to_string_lossy(), e),
            };
        // let mut f = File::open(&path).unwrap();

        // let mut rdr = BufReader::with_capacity(1024*1024*16, f);
        let mut lines = 0;
        let mut bc = 0usize;

        println!("reading...");
        if !_slow  {
            println!("fast read");
            loop {
                // let len = {
                //         let buf = rdr.fill_buf().unwrap();
                //         lines += buf[..].into_iter().filter(|&&b| b == b'\n').count();
                //         buf.len()
                // };
                // rdr.consume(len);

                // if len <= 0 { break; }

                match f.read(&mut buf[..]) {
                    Ok(0) => {
                        println!("zero returned");
                        break;
                    }
                    Ok(len) => {
                        lines += bytecount::naive_count_32(&buf[0..len], b'\n');
                        bc += len;
                        //lines += buf[0..len].into_iter().filter(|&&b| b == b'\n').count();
                        // for i in 0..len {
                        //         if buf[i] == b'\n' { lines += 1}
                        // }
                    }
                    Err(_) => break,
                };
            }
        } else {
            // Option<Result<String, Error>>
            /*
            let mut rdr = BufReader::with_capacity(1024*1024*1, f);
            loop {
                match fast_read_line(&mut rdr) {
                    Some(x) => {
                        match x {
                            Ok(s) => {
                                lines +=1;
                                bc += s.len();
                            },
                            Err(e) => panic!("fast read line {}", e),
                        }
                    },
                    None => break
                };
            }
            */

            /*
            loop {
                let mut myvec = vec![];
                match f.read_until(b'\n', &mut myvec) {
                    Ok(s) => {
                        //println!("{:?}", myvec);
                        if s == 0 { break; }
                        let string = unsafe { String::from_utf8_unchecked(myvec) };
                        lines +=1;
                        bc += string.len();
                        //alllines.push(string);
                    },
                    _ => break,
                }
            }
            */

            //for x in alllines { println!("{}", x);}

            println!("reading primitive");
            let mut rdr = BufReader::new(f);
            let mut mystr = String::new();
            loop{
                match rdr.read_line(&mut mystr) {
                    Ok(s) => {  lines += 1; bc += bc}
                    Ok(0) => break,
                    Err(e) => panic!("error {}",e),
                }
            }
            /*
            for _line in rdr.lines() {
                lines += 1;
                bc += _line.unwrap().len();
            }
            */

        } // 💯
        println!("{} had {} lines and {} bytes", path.to_str().unwrap(), lines, bc);
    }
}

pub fn mainstdin(slow: bool) {
    let mut buffer = [0u8; 1024 * 128];
    //let mut stdin = ::std::io::stdin();

    let mut wc = 0usize;
    let mut bc = 0usize;

    if !slow {
        //let mut stdinl = stdin.lock();
        loop {
            match ::std::io::stdin().read(&mut buffer) {
                Ok(0) => {
                    break;
                }
                Ok(len) => {
                    wc += bytecount::naive_count_32(&buffer[0..len], b'\n');
                    bc += len;
                    //wc += buffer[0..len].into_iter().filter(|&&b| b == b'\n').count();
                }
                Err(err) => {
                    panic!("{}", err);
                }
                _ => { break; }
            }
        }
    } else {
        /*
        for _line in ::std::io::stdin().lines() {
            wc += 1;
            bc += _line.unwrap().len();
        }
        */
    }
    println!("lines: {}  bytes: {} ", wc, bc);
}
pub fn fast_read_line(rdr : &mut BufRead,  mut buf: &mut Vec<u8>) -> Option<Result<String, Error>> {

    let result = { rdr.read_until(b'\n', &mut buf) };
    let newbuf = buf.clone();
    match result {
        Ok(0) => Option::None,
        Ok(s) => {
            //let buf = buf.into_inner();
            //let buf2 = *buf;
            let line2 = unsafe { String::from_utf8(newbuf).unwrap() } ;
            buf.clear();
            //Option::Some(Ok(String::from(string)))
            Option::Some(Ok(line2))
        },
        Err(e) => Some(Err(e))
    }
}


pub fn fast_read_line__(rdr : &mut BufRead) -> Option<Result<String, Error>> {
    let mut myvec = vec![];
    match rdr.read_until(b'\n', &mut myvec) {
        Ok(0) => Option::None,
        Ok(s) => {
            let string = unsafe { String::from_utf8_unchecked(myvec) };
            Option::Some(Ok(string))
        },
        Err(e) => Some(Err(e))
    }
}

fn main() {}
