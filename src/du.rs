#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate users;

use std::fs;
use std::env::args;
//use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::collections::BTreeMap;
use std::os::linux::fs::MetadataExt;
use users::{get_user_by_uid, get_current_uid};


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

fn track_top_n(map: &mut BTreeMap<u64, String>, path: &Path, size: u64, limit: usize) -> bool {
    if size <= 0 {
        return false
    }

    if limit > 0 {
        if map.len() < limit {
            let spath = String::from(path.to_str().unwrap());
            map.insert(size, spath);
            return true
        } else {
            let lowest = match map.iter().next() {
                Some( (l,p) ) => *l,
                None => 0u64
            };
            if lowest < size {
                map.remove(&lowest);
                let spath = String::from(path.to_str().unwrap());
                map.insert(size, spath);
            }
        }
    }
    return false
}

fn walk_dir(dir: &Path, depth: u32, user_map: &mut BTreeMap<u32, u64>, mut top_dir: &mut BTreeMap<u64,String>) -> u64 {
    let itr = fs::read_dir(dir);
    let mut this_tot = 0;
    match itr {
        Ok(e) => {
            let mut local_tot = 0u64;
            for e in e {
                let e = e.unwrap();
                let meta = e.metadata().unwrap();
                let stat = meta.as_raw_stat();
                let p = e.path();
                if meta.is_file() {
                    let s = meta.len();
                    this_tot += s;
                    local_tot += s;
                    let uid = meta.st_uid();
                    *user_map.entry(uid).or_insert(0) += s;
                } else if meta.is_dir() {
                    this_tot += walk_dir(&p, depth+1, user_map, top_dir);
                }

            }
            track_top_n(&mut top_dir, &dir, local_tot, 10);
        },
        Err(e) =>
            println!("Cannot read dir: {}, error: {} so skipping ", &dir.to_str().unwrap(), &e),
    }
    this_tot
}

fn main() {
    let spath = args().nth(1).expect("missing 1st arg for top path to scan").to_string();
    let path = Path::new(& spath);
    if path.is_dir() {
        let mut user_map: BTreeMap<u32, u64> = BTreeMap::new();
        let mut top_dir: BTreeMap<u64, String> = BTreeMap::new();

        let total = walk_dir(&path, 0, &mut user_map, &mut top_dir);
        println!("Total scanned: {}", greek(total as f64));

        println!("\nSpace per user");
        for (k, v) in &user_map {
            match get_user_by_uid(*k) {
                None => println!("uid{:7} {} ", *k, greek(*v as f64)),
                Some(user) => println!("{:10} {} ", user.name(), greek(*v as f64)),
            }

        }
        println!("\nTop dir with space usage directly inside them");
        for (k, v) in top_dir.iter().rev() {
            println!("{:10} {}", greek(*k as f64),v);
        }

    } else {
        println!("path {} is not a directory", spath);
    }
}
