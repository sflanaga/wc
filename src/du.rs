#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate users;

use std::fs;
use std::env::args;
//use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::os::linux::fs::MetadataExt;
use users::{get_user_by_uid, get_current_uid};

type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;
use std::fmt;


mod util;
use util::{greek};

#[derive(Eq, Debug)]
struct TrackedPath {
    size: u64,
    path: PathBuf
}

impl Ord for TrackedPath {
    fn cmp(&self, other: &TrackedPath) -> Ordering {
        self.size.cmp(&other.size).reverse()
    }
}

impl PartialOrd for TrackedPath {
    fn partial_cmp(&self, other: &TrackedPath) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TrackedPath {
    fn eq(&self, other: &TrackedPath) -> bool {
        self.size == other.size
    }
}



fn track_top_n(map: &mut BTreeMap<u64, PathBuf>, path: &Path, size: u64, limit: usize) -> bool {
    if size <= 0 {
        return false;
    }

    if limit > 0 {
        if map.len() < limit {
            let spath = path.to_path_buf();
            map.insert(size, spath);
            return true
        } else {
            let lowest = match map.iter().next() {
                Some( (l,p) ) => *l,
                None => 0u64
            };
            if lowest < size {
                map.remove(&lowest);
                let spath = path.to_path_buf();
                map.insert(size, spath);
            }
        }
    }
    return false;
}

fn track_top_n2(heap: &mut BinaryHeap<TrackedPath>, p: &Path, s: u64, limit: usize) -> bool {
    if s <= 0 {
        return false;
    }

    if limit > 0 {
        if heap.len() < limit {
            heap.push(TrackedPath{size: s, path: p.to_path_buf()});
            return true
        } else {
            if heap.peek().expect("cannot peek when the size is greater than 0!?").size < s {
                heap.pop();
                heap.push(TrackedPath{size: s, path: p.to_path_buf()});
                return true;
            }
        }
    }
    return false;
}

fn walk_dir(limit: usize, dir: &Path, depth: u32,
    user_map: &mut BTreeMap<u32, u64>,
    mut top_dir: &mut BinaryHeap<TrackedPath>,
    mut top_cnt_dir: &mut BinaryHeap<TrackedPath>,
    mut top_cnt_file: &mut BinaryHeap<TrackedPath>,
    mut top_dir_overall: &mut BinaryHeap<TrackedPath>,
    mut top_files: &mut BinaryHeap<TrackedPath>) -> GenResult<(u64,u64)> {
    let itr = fs::read_dir(dir);
    let mut this_tot = 0;
    let mut this_cnt = 0;
    match itr {
        Ok(itr) => {
            let mut local_tot = 0u64;
            let mut local_cnt_file = 0u64;
            let mut local_cnt_dir = 0u64;
            for e in itr {
                let e = e?;
                let meta = e.metadata()?;
                let p = e.path();
                if meta.is_file() {
                    let s = meta.len();
                    this_tot += s;
                    local_tot += s;
                    let uid = meta.st_uid();
                    *user_map.entry(uid).or_insert(0) += s;
                    local_cnt_file += 1;
                    this_cnt +=1;
                    track_top_n2(&mut top_files, &p, s, limit); // track single immediate space
                    // println!("{}", p.to_str().unwrap());
                } else if meta.is_dir() {
                    local_cnt_dir += 1;
                    //let (that_tot, that_cnt) = walk_dir(limit, &p, depth+1, user_map, top_dir, top_cnt_dir, top_cnt_file, top_dir_overall, top_files)?;
                    match walk_dir(limit, &p, depth+1, user_map, top_dir, top_cnt_dir, top_cnt_file, top_dir_overall, top_files) {
                        Ok( (that_tot, that_cnt) ) => { this_tot += that_tot; this_cnt += that_cnt; },
                        Err(e) => eprint!("error trying walk {}, error = {} but continuing",p.to_string_lossy(), e),
                    };
                }
            }
            track_top_n2(&mut top_dir, &dir, local_tot, limit); // track single immediate space
            track_top_n2(&mut top_cnt_dir, &dir, local_cnt_dir, limit); // track dir with most # of dir right under it
            track_top_n2(&mut top_cnt_file, &dir, local_cnt_file, limit); // track dir with most # of file right under it
            track_top_n2(&mut top_dir_overall, &dir, this_tot, limit); // track top dirs overall - main will be largest
        },
        Err(e) =>
            eprintln!("Cannot read dir: {}, error: {} so skipping ", &dir.to_str().unwrap(), &e),
    }
    return Ok( (this_tot, this_cnt) );
}

fn run() -> GenResult<()> {
    let limit = args().nth(1).expect("missing 1st arg which is the number of top X to track").to_string().parse::<usize>().unwrap();
    let spath = args().nth(2).expect("missing 2nd arg for top directory to scan").to_string();
    let path = Path::new(& spath);
    if path.is_dir() {
        let mut user_map: BTreeMap<u32, u64> = BTreeMap::new();

        let mut top_dir: BinaryHeap<TrackedPath> = BinaryHeap::new();
        let mut top_cnt_dir: BinaryHeap<TrackedPath> = BinaryHeap::new();
        let mut top_cnt_file: BinaryHeap<TrackedPath> = BinaryHeap::new();
        let mut top_dir_overall: BinaryHeap<TrackedPath> = BinaryHeap::new();
        let mut top_files: BinaryHeap<TrackedPath> = BinaryHeap::new();

        let (total, count) = match walk_dir(limit, &path, 0, &mut user_map, &mut top_dir,  &mut top_cnt_dir,  &mut top_cnt_file,  &mut top_dir_overall, &mut top_files) {
            Ok( (that_tot, that_cnt) ) => { (that_tot, that_cnt) },
            Err(e) => {
                eprint!("error trying walk top dir {}, error = {} but continuing",path.to_string_lossy(), e);
                (0,0)
            }

        };
        //let (total,count) = walk_dir(limit, &path, 0, &mut user_map, &mut top_dir,  &mut top_cnt_dir,  &mut top_cnt_file,  &mut top_dir_overall, &mut top_files)?;

        println!("Total scanned: {} and {} files", greek(total as f64), count);

        println!("\nSpace used per user");
        for (k, v) in &user_map {
            match get_user_by_uid(*k) {
                None => println!("uid{:7} {} ", *k, greek(*v as f64)),
                Some(user) => println!("{:10} {} ", user.name(), greek(*v as f64)),
            }

        }
        println!("\nTop dir with space usage directly inside them");

        // loop {
        //     match top_dir.pop() {
        //         None => break,
        //         Some(v) => println!("{:10} {}", greek(v.size as f64),v.path.to_string_lossy()),
        //     }
        // }
        for v in top_dir.into_sorted_vec() {
            println!("{:10} {}", greek(v.size as f64),v.path.to_string_lossy());
        }


        println!("\nTop dir ");
        for v in top_dir_overall.into_sorted_vec() {
            println!("{:10} {}", greek(v.size as f64),v.path.to_string_lossy());
        }


        println!("\nTop dir count with files directly inside it");
        for v in top_cnt_file.into_sorted_vec() {
            println!("{:10} {}", v.size,v.path.to_string_lossy());
        }

        println!("\nTop dir count with directories directly inside it");
        for v in top_cnt_dir.into_sorted_vec() {
            println!("{:10} {}", v.size,v.path.to_string_lossy());
        }

        println!("\nTop sized files");
        for v in top_files.into_sorted_vec() {
            println!("{:10} {}", greek(v.size as f64),v.path.to_string_lossy());
        }
    } else {
        println!("path {} is not a directory", spath);
    }
    Ok( () )
}

fn main() {
    if let Err(err) = run() {
        println!("uncontrolled error: {}", &err);
        std::process::exit(1);
    }
}
