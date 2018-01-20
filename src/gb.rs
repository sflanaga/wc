#![allow(unused_imports)]

use std::io::prelude::*;
use std::fs::OpenOptions;

use std::string::String;
use std::env::args;
use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;
use std::fs;

use std::borrow::Borrow;
use std::path::Path;
use std::thread;

use std::io::Lines;
use std::time::Instant;

use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

// use std::io::StdinLock;
// use std::io::StdinLock::read;

#[derive(Debug)]
struct KeySum {
    unique_values: BTreeSet<String>,
    count : u64
}


fn main() {
    for x in args() { println!("{:?}", x);}
    let d = args().nth(1).unwrap().chars().nth(0).unwrap();
    let key_fields = args().nth(2).unwrap();
    let unique_count = args().nth(3).unwrap();
    let unique_c_field = match unique_count.parse::<i32>() {
        Ok(x) => x,
        Err(e) => { println!("cannot parse option {} so skipping the unique key thing here, but the error was: {}", unique_count, e); -1 }
    };

    let key_fields : Box<Vec<usize>> = Box::new(key_fields.split(",").map(|x| x.parse::<usize>().unwrap()).collect());

    let mut maxfield = if unique_c_field < 0 { 0 } else { unique_c_field as usize};
    for x in key_fields.iter() {
        if x+1 > maxfield { maxfield = x+1; }
    }

    println!("maxfield: {}", maxfield);

    let mut hm : BTreeMap<String, KeySum> = BTreeMap::new();

    if args().len() >= 5 {

        for p in args().skip(4) {
            let metadata = fs::metadata(p.clone()).unwrap(); //ok().expect(format!("could not get meta data for file {}", forerror_msg.to_string())); // unwrap();
            print!("processing file: {} of {} ... ", p, greek(metadata.len() as f64),);
            std::io::stdout().flush().ok().expect("Could not flush stdout");
            let start_f = Instant::now();

            let f = match OpenOptions::new()
                    .read(true)
                    .write(false)
                    .create(false)
                    .open(p.clone())
                    {
                        Ok(f) => f,
                        Err(e) => panic!("cannot open file \"{}\" due to this error: {}",p, e),
                    };
            let mut rdr = BufReader::with_capacity(1024*1024*1,f);

            let (_bc,lines) = read_file(&mut rdr, &mut hm, d, & key_fields, unique_c_field, maxfield);
//            let (_bc,lines) = read_file_until(&mut rdr, &mut hm, d, & key_fields, unique_c_field);

            let elapsed = start_f.elapsed();
            let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
            let rate : f64= metadata.len() as f64 / sec;
            println!(" had {} lines, time: {} secs rate: {}/s",lines, sec, greek(rate));

        } // per file loop
    } else { // read stdin
        let start_f = Instant::now();

        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        println!("reading stdin...");
        let (bc,lines) = read_file(&mut handle, &mut hm, d, &key_fields, unique_c_field,maxfield);
        let elapsed = start_f.elapsed();
        let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
        let rate : f64= bc as f64 / sec;
        println!(" had {} lines, time: {} secs rate: {}/s",lines, sec, greek(rate));
    }

    for (ff,cc) in &hm {
        println!("{} <=> {}  {}", ff,cc.count, cc.unique_values.len());
    }
}

fn read_file(rdr: &mut BufRead, hm : &mut BTreeMap<String, KeySum>, delimiter: char, key_fields : & Vec<usize>, unique_c_field: i32, maxfield: usize) -> (usize,usize) {
    let mut lines = 0;
    let mut bytes = 0usize;

    let mut ss : String = String::with_capacity(256);
    let mut it : [String;32] = Default::default();
    let mut unique_count_field : String = String::with_capacity(256);

    for _line in rdr.lines() {
        lines += 1;
        let l = _line.unwrap();
        bytes += l.len()+1;

        for i in 0..(key_fields.len()) { it[i].clear(); }

        let mut pushed_count = 0;

        unique_count_field.clear();
        for (i,field) in l.split(delimiter).take(maxfield).enumerate() {
            for(j,index) in key_fields.iter().enumerate() {
                if *index == i {
                    it[j].push_str(field);
                    pushed_count += 1;
                }

                if unique_c_field as usize == i {
                    unique_count_field.push_str(field);
                }
            }
        }

        ss.clear();

        for i in 0..pushed_count {
            if i != pushed_count -1 {
                ss.push_str(&it[i]);
                ss.push_str("|");
            } else { ss.push_str(&it[i]); }
        }

        {
            let v = hm.entry(ss.clone()).or_insert(KeySum{ count : 0, unique_values: BTreeSet::new() });
            v.count = v.count +1;
            if unique_count_field.len() > 0 {
                v.unique_values.insert(unique_count_field.clone());
            }

        }


    } // lines loop
    (bytes,lines)
} // END read_file



fn read_file_until(rdr: &mut BufRead, hm : &mut BTreeMap<String, KeySum>, delimiter: char, key_fields : & Vec<usize>, unique_c_field: usize) -> (usize,usize) {
    let mut lines = 0;
    let mut bytes = 0usize;

    //let mut ss : String = String::with_capacity(256);
    let mut it : [String;32] = Default::default();
    let mut unique_count_field : String = String::with_capacity(256);


    loop {
        let mut buf = Vec::new(); // &mut vec![];
        let s = rdr.read_until(b'\n', &mut buf).unwrap();
        if s > 0 {
//            let line = unsafe { String::from_utf8_unchecked(buf) };
            let line = String::from_utf8(buf).unwrap();
            lines += 1;
            bytes += line.len()+1;

            for i in 0..(key_fields.len()) { it[i].clear(); }

            let mut pushed_count = 0;

            unique_count_field.clear();

            for (i,field) in line.split(delimiter).enumerate() {
                for(j,index) in key_fields.iter().enumerate() {
                    if *index == i {
                        it[j].push_str(field);
                        pushed_count += 1;
                    }

                    if unique_c_field == i {
                        unique_count_field.push_str(field);
                    }
                }
            }

            //ss.clear();
            let mut ss = String::with_capacity(64);
            for i in 0..pushed_count {
                if i != pushed_count -1 {
                    ss.push_str(&it[i]);
                    ss.push_str("|");
                } else { ss.push_str(&it[i]); }
            }

            {
                let v = hm.entry(ss.clone()).or_insert(KeySum{ count : 0, unique_values: BTreeSet::new() });
                v.count = v.count +1;
                if unique_count_field.len() > 0 {
                    v.unique_values.insert(unique_count_field.clone());
                }

            }




        } else { break; }

    }

    (bytes,lines)
} // END read_file




pub fn greek(v: f64) -> String {
	const GR_BACKOFF: f64 = 24.0;
	const GROWTH: f64 = 1024.0;
	const KK : f64 = GROWTH;
	const MM : f64 = KK*GROWTH;
	const GG: f64 = MM*GROWTH;
	const TT: f64 = GG*GROWTH;
	const PP: f64 = TT*GROWTH;

	let a = v.abs();
		// println!("hereZ {}  {}  {}", v, MM-(GR_BACKOFF*KK), GG-(GR_BACKOFF*MM));
	let t = if a > 0.0 && a < KK - GR_BACKOFF {
		(v, "B")
	} else if a >= KK - GR_BACKOFF && a < MM-(GR_BACKOFF*KK) {
		// println!("here {}", v);
		(v/KK, "K")
	} else if a >= MM-(GR_BACKOFF*KK) && a < GG-(GR_BACKOFF*MM) {
		// println!("here2 {}  {}  {}", v, MM-(GR_BACKOFF*KK), GG-(GR_BACKOFF*MM));
		(v/MM, "M")
	} else if a >= GG-(GR_BACKOFF*MM) && a < TT-(GR_BACKOFF*GG) {
		// println!("here3 {}", v);
		(v/GG, "G")
	} else if a >= TT-(GR_BACKOFF*GG) && a < PP-(GR_BACKOFF*TT) {
		// println!("here4 {}", v);
		(v/TT, "T")
	} else {
		// println!("here5 {}", v);
		(v/PP, "P")
	};

	let mut s = format!("{}", t.0);
	s.truncate(4);
	if s.ends_with(".") {
		s.pop();
	}

	format!("{}{}", s, t.1)
}
