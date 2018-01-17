#![allow(unused_imports)]

use std::io::prelude::*;
use std::fs::OpenOptions;

use std::io::Error;
use std::string;
use std::string::String;
use std::env::args;
use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;

use std::borrow::Borrow;
use std::path::Path;
use std::thread;

use std::io::Lines;

use std::collections::HashMap;

// use std::io::StdinLock;
// use std::io::StdinLock::read;

fn main() {
    let p = args().nth(1).unwrap();
    let mut f = match OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(p.clone())
            {
                Ok(f) => f,
                Err(e) => panic!("cannot open file due to this error: {}", e),
            };
    let find = String::from(args().nth(2).unwrap());

    let mut hm : HashMap<String, u32> = HashMap::new();

    let mut lines = 0;
    let mut bc = 0usize;
    let mut rdr = BufReader::new(f);
    let mut m = 0;
    let myvec : &mut Vec<u8> = &mut vec![];

    loop {
        let s = match fast_read_line(&mut rdr, myvec) {
            Some(Ok(s)) => s,
            None => break,
            Some(Err(e)) => panic!("error in loop {}", e),
        };
        lines += 1;
        bc += s.len();
    }

    // for _line in rdr.lines() {
    //     lines += 1;
    //     let l = _line.unwrap();
    //     bc += l.len();
    //     let x = String::from(l.split(' ').nth(0).unwrap());

    //     *hm.entry(x).or_insert(1) += 1;

    // }
    println!("max {}",m);

    for (ff,cc) in &hm {
        println!("{} ||| {}", ff,cc);
    }
/*

    println!("reading {}",&p);
    let mut mystr = String::new();
    loop{
        match rdr.read_line(&mut mystr) {
            Ok(0) => break,
            Ok(s) => { 
                /* println!("{} ", mystr);*/
                lines += 1; 
                bc += s; 
                if  mystr.contains(&find)  {
                    println!("found {}", mystr);
                }
                mystr.clear();
            },
            Err(e) => panic!("error here {}", e),
        }
    }
*/
/*    let mut rdr = BufReader::with_capacity(1024*1024*1, f);
    let myvec : &mut Vec<u8> = &mut vec![];
    loop {
        match fast_read_line(&mut rdr, myvec) {
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
    println!("{} lines {} bytes", lines,bc);
}

pub fn fast_read_line(rdr : &mut BufRead,  mut buf: &mut Vec<u8>) -> Option<Result<String, Error>> {
    
    let result = { rdr.read_until(b'\n', &mut buf) };
    let newbuf = buf.clone();
    match result {
        Ok(0) => Option::None,
        Ok(s) => {
            //let buf = buf.into_inner();
            //let buf2 = *buf;
            let line2 = String::from_utf8(newbuf).unwrap();
            buf.clear();
            //Option::Some(Ok(String::from(string)))
            Option::Some(Ok(line2))
        },
        Err(e) => Some(Err(e))
    }
}


