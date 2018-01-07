#![allow(unused_imports)]

extern crate bytecount;
use std::borrow::Borrow;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::thread;
// use std::io::StdinLock;
// use std::io::StdinLock::read;
use std::io::Read;
use std::io::BufReader;

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
        if ( !_slow ) {

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
            let mut rdr = BufReader::with_capacity(1024*1024*1, f);
            //let mut alllines = vec![];
            loop {
                let mut myvec = vec![];
                match rdr.read_until(b'\n', &mut myvec) {
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
            //for x in alllines { println!("{}", x);}

/*
            let mut rdr = BufReader::with_capacity(1024*1024*1, f);
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
