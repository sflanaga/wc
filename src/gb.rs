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

enum FieldType { 
    key,
}



fn main() {
    let d = args().nth(1).unwrap().chars().nth(0).unwrap();
    let key_fields = args().nth(2).unwrap();
    let unique_count = args().nth(3).unwrap();

    let unique_c_field = match unique_count.parse::<usize>() {
        Ok(x) => x,
        Err(e) => { println!("cannot parse option {} so skipping the unique key thing here, but the error was: {}", unique_count, e); 1000000 }
    };

    

    let myvec : &mut Vec<u8> = &mut vec![];

    let kf = &mut vec![0usize;0];
    for x in key_fields.split(",") {
        kf.push(x.parse::<usize>().unwrap());
    }
    let mut hm : BTreeMap<String, KeySum> = BTreeMap::new();

    for p in args().skip(4) {
        let metadata = fs::metadata(p.clone()).unwrap();
        print!("processing file: {} of {} ... ", p, greek(metadata.len() as f64),);
        std::io::stdout().flush().ok().expect("Could not flush stdout");
        let start_f = Instant::now();

        let mut f = match OpenOptions::new()
                .read(true)
                .write(false)
                .create(false)
                .open(p.clone())
                {
                    Ok(f) => f,
                    Err(e) => panic!("cannot open file \"{}\" due to this error: {}",p, e),
                };
        let mut lines = 0;
        let mut bc = 0usize;

        let mut rdr = BufReader::new(f);
        let myvec : &mut Vec<u8> = &mut vec![];

        let mut ss : String = String::with_capacity(256);
        let mut it : [String;10] = Default::default();
        let mut uf : String = String::with_capacity(256);
        for _line in rdr.lines() {

            lines += 1;
            let l = _line.unwrap();
            bc += l.len();

            // 4,1,2
            uf.clear();
            for (i,f) in l.split(d).enumerate() {
                for(j,index) in kf.iter().enumerate() {
                    
                    if *index == i {
                        it[j].clear();
                        it[j].push_str(f); // = f.to_string();
                    }

                    if unique_c_field == i {
                        // println!("u:  {}", f);
                        uf.push_str(f);
                    }
                }
            }

            ss.clear();
            for i in 0..(kf.len()-1) {
                if i != kf.len()-1 {
                    ss.push_str(&it[i]);
                    ss.push_str("|");
                } else { ss.push_str(&it[i]); }
            }

            for pp in it.iter() { ss.push_str(pp); }
            {
                let v = hm.entry(ss.clone()).or_insert(KeySum{ count : 0, unique_values: BTreeSet::new() });
                v.count = v.count +1;
                if uf.len() > 0 {
                    v.unique_values.insert(uf.clone());
                }

            }
        } // lines loop
        let elapsed = start_f.elapsed();
        let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
        let rate : f64= metadata.len() as f64 / sec;
        println!(" had {} lines, time: {} secs rate: {}/s",lines, sec, greek(rate));

    } // per file loop

    for (ff,cc) in &hm {
        println!("{} <=> {}  {}", ff,cc.count, cc.unique_values.len());
    }        


}

/*

fn read_file(rdr: BufReader, hm : &mut BTreeMap<String, KeySum>, filename: String) {

        // let mut f = match OpenOptions::new()
        //         .read(true)
        //         .write(false)
        //         .create(false)
        //         .open(filename.clone())
        //         {
        //             Ok(f) => f,
        //             Err(e) => panic!("cannot open file \"{}\" due to this error: {}",filename, e),
        //         };
        let mut lines = 0;
        let mut bc = 0usize;

        let mut rdr = BufReader::new(f);
        let myvec : &mut Vec<u8> = &mut vec![];

        let mut ss : String = String::with_capacity(256);
        let mut it : [String;10] = Default::default();
        let mut uf : String = String::with_capacity(256);
        for _line in rdr.lines() {

            lines += 1;
            let l = _line.unwrap();
            bc += l.len();

            // 4,1,2
            uf.clear();
            for (i,f) in l.split(d).enumerate() {
                for(j,index) in kf.iter().enumerate() {
                    
                    if *index == i {
                        it[j].clear();
                        it[j].push_str(f); // = f.to_string();
                    }

                    if unique_c_field == i {
                        // println!("u:  {}", f);
                        uf.push_str(f);
                    }
                }
            }

            ss.clear();
            for i in 0..(kf.len()-1) {
                if i != kf.len()-1 {
                    ss.push_str(&it[i]);
                    ss.push_str("|");
                } else { ss.push_str(&it[i]); }
            }

            for pp in it.iter() { ss.push_str(pp); }
            {
                let v = hm.entry(ss.clone()).or_insert(KeySum{ count : 0, unique_values: BTreeSet::new() });
                v.count = v.count +1;
                if uf.len() > 0 {
                    v.unique_values.insert(uf.clone());
                }

            }
        } // lines loop
        let elapsed = start_f.elapsed();
        let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
        let rate : f64= metadata.len() as f64 / sec;
        println!(" had {} lines, time: {} secs rate: {}/s",lines, sec, greek(rate));
}
*/



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
	// match v {
	// 	(std::ops::Range {start: 0, end: (KK - GR_BACKOFF)}) => v,
	// 	//KK-GR_BACKOFF .. MM-(GR_BACKOFF*KK)			=> v,
	// 	_ => v,
	// }
}
