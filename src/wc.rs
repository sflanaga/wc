#![allow(unused_imports)]

extern crate bytecount;
use std::borrow::Borrow;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::prelude::*;
// use std::io::StdinLock;
// use std::io::StdinLock::read;
use std::io::Read;

pub fn mainfile<'a, I>(filenames: I, _slow: bool) 
where
    I: Iterator<Item = &'a str>,
//     I: IntoIterator,
//     I::Item: Borrow<&'a str>,
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
        println!("reading...");
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
                    //lines += buf[0..len].into_iter().filter(|&&b| b == b'\n').count();
                    // for i in 0..len {
                    //         if buf[i] == b'\n' { lines += 1}
                    // }
                }
                Err(_) => break,
            };
        }
        println!("{} had {} lines", path.to_str().unwrap(), lines);
    }
}

pub fn mainstdin(slow: bool) {
    let mut buffer = [0u8; 1024 * 128];
    let stdin = ::std::io::stdin();
    
    let mut wc = 0usize;
    if !slow {
        let mut stdin = stdin.lock();
        loop {
            match stdin.read(&mut buffer) {
                Ok(0) => {
                    break;
                }
                Ok(len) => {
                    wc += bytecount::naive_count_32(&buffer[0..len], b'\n');
                    //wc += buffer[0..len].into_iter().filter(|&&b| b == b'\n').count();
                }
                Err(err) => {
                    panic!("{}", err);
                }
            }
        }
    } else {
        //let mut stdin = stdin.lock();
        loop {
            for _line in stdin.lock().lines() {
                wc += 1;
            }
        }
    }
    println!("{}", wc);
}
