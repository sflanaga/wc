#[macro_use] extern crate log;
extern crate env_logger;
extern crate bytecount;
use std::env;
use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::prelude::*;
use std::time::Instant; 
use bytecount::*;

fn main() {
        let now = Instant::now();
        if env::args_os().count() > 1 {
                mainfile();
        } else {
                mainstdin();
        }
        let elapsed = now.elapsed();
        let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
        println!("{} seconds ", sec);
}


fn mainfile() {
        env_logger::init().unwrap();
        warn!("logging setup");
        let mut buf: [u8; 1024*128] = [0; 1024*128];



        //let mut buf = Vec::with_capacity(1024*1024*32).into_boxed_slice();
        // buf = v.

        for arg  in env::args_os().skip(1) {
                let a = arg.into_string().unwrap();
                let path = Path::new(&a);
                let mut f = OpenOptions::new().read(true).write(false).create(false).open(&path).unwrap();
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

                        match f.read( & mut buf[..]) {
                                Ok(0) => {warn!("zero returned"); break;},
                                Ok(len) => {

                                        lines += bytecount::naive_count_32(&buf[0..len], b'\n');
                                        //lines += buf[0..len].into_iter().filter(|&&b| b == b'\n').count();
                                        // for i in 0..len {
                                        //         if buf[i] == b'\n' { lines += 1}
                                        // }
                                },
                                Err(_) => break
                        };
                }
                println!("{} had {} lines", path.to_str().unwrap(), lines);

    }
}


fn mainstdin() {
    let mut buffer = [0u8; 1024*128];
    let stdin = ::std::io::stdin();
    let mut stdin = stdin.lock();
    let mut wc = 0usize;
    loop {
        match stdin.read(&mut buffer) {
            Ok(0) => {
                break;
            },
            Ok(len) => {
                wc += bytecount::naive_count_32(&buffer[0..len], b'\n');
                //wc += buffer[0..len].into_iter().filter(|&&b| b == b'\n').count();
            },
            Err(err) => {
                panic!("{}", err);
            },
        }
    };
    println!("{}", wc);
}



fn main3() {
       let mut buf: [u8; 4096] = [0; 4096];
        for arg  in env::args_os().skip(1) {
                let a = arg.into_string().unwrap();
                let path = Path::new(&a);
                let mut f = File::open(&path).unwrap();
                let mut lines = 0usize;
                loop {
                        match f.read(&mut buf[..]) {
                                Ok(0) => {
                                        break;
                                },
                                Ok(len) => {
                                        lines += buf[0..len].into_iter().filter(|&&b| b == b'\n').count();
                                },
                                Err(err) => {
                                        panic!("{}", err);
                                },
                        }
                }
                println!("{} had {} lines", path.to_str().unwrap(), lines);
        }
}

fn main1() {
        for arg  in env::args_os().skip(1) {
                let a = arg.into_string().unwrap();
                let path = Path::new(&a);
                let f = File::open(&path);
                let file = BufReader::new(f.unwrap());
                let mut count = 0;
                for _ in file.lines() {
                        count += 1;
                }
                //f.close();
                println!("{} had {} lines", path.to_str().unwrap(), count);
        }

}

