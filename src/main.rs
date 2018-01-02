extern crate clap;
use clap::{App, Arg, SubCommand};

// #[macro_use] extern crate log;
// extern crate env_logger;
extern crate bytecount;
use std::borrow::Borrow;
use std::env::args;
use std::env::Args;
use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::prelude::*;
use std::time::Instant;

fn main() {
    println!("{}", args().nth(0).unwrap());

    let matches = App::new("tester")
        .version("1.0")
        .author("Steve F<disposesmf@gmail.com>")
        .about("test things")
        .subcommand(
            SubCommand::with_name("wc")
                .about("word / line couter")
                .version("0.1")
                .author("Someone E. <someone_else@other.com>")
                //.arg_from_usage("-d, --debug 'Print debug information'"),
                .arg_from_usage("[input]... 'an optional input file to use'"),
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("wc") {
        println!("sub wc...");
        if matches.is_present("input") {
                println!("input files");
                let inp_files = matches.values_of("input").unwrap();
                /*
                println!("{:?}", inp_files);
                let inp_files2 = inp_files.clone();
                inp_files.into_iter().enumerate().for_each(|(i,f)| println!("file {} == {}", i, f));
                for (i,f) in inp_files2.enumerate() {
                        println!("file {} == {}", i, f);
                }
                */
                mainfile(inp_files);
        } else {
                mainstdin();
        }
        
        if matches.is_present("debug") {
            println!("wc with debug");
        } else {
            println!("wc no debug");
        }
    }
/*
    if args().count() > 10 {
        if args().nth(1).unwrap() == "wc" {
            let now = Instant::now();
            if args().count() > 1 {
                mainfile();
            } else {
                mainstdin();
            }
            let elapsed = now.elapsed();
            let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
            println!("{} seconds ", sec);
        }
    }
    */
}


fn mainfile<'a, I>(filenames: I) 
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
        let mut f = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(&path)
            .unwrap();
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

fn mainstdin() {
    let mut buffer = [0u8; 1024 * 128];
    let stdin = ::std::io::stdin();
    let mut stdin = stdin.lock();
    let mut wc = 0usize;
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
    println!("{}", wc);
}
