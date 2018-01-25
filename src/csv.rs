#![allow(unused_imports)]


use std::io::prelude::*;
use std::fs::OpenOptions;
use std::fs;

use std::string::String;
use std::env::args;
use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Error;
use std::fs::File;

use std::borrow::Borrow;
use std::path::Path;
use std::thread;

use std::io::Lines;
use std::time::Instant;

use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
extern crate csv;



#[derive(Debug)]
struct KeySum {
//    unique_values: BTreeSet<String>,
    count : u64
}
fn main() {
    if let Err(err) = csv() {
        println!("error: {:?}", &err);
        std::process::exit(1);
    }
}

fn csv() -> Result<(),std::io::Error> {

    let mut key_fields = vec![];
    let mut unique_fields = vec![];
    let mut ii = 1..args().len();
    let mut delimiter : char = ',';

    let argv : Vec<String> = args().skip(1).map( |x| x).collect();
    let mut filelist = &mut vec![];
    let mut verbose = false;
    let mut hasheader = false;
    let mut i = 0;
    while i < argv.len() {
        match &argv[i][..] {
            "-f" => { // field list processing
                i += 1;
                key_fields.splice(.., (&argv[i][..]).split(",").map( |x| x.parse::<usize>().unwrap()) );
            },
            "-u" => { // unique count AsMut
                i += 1;
                unique_fields.splice(.., (&argv[i][..]).split(",").map( |x| x.parse::<usize>().unwrap()) );
            },
            "-d" => { // unique count AsMut
                i += 1;
                delimiter = argv[i].as_bytes()[0] as char;
            },
            "-v" => { // write out AsMut
                verbose = true;
                println!("writing stats and other info ON")
            },
            "-h" => { // write out AsMut
                hasheader = true;
            },
            x => { println!("adding filename {} to scan", x); filelist.push(x); }
        }

        i += 1;
    }
    let maxfield = 1;

    if verbose {
        println!("\tdelimiter: {}", delimiter);
        println!("\theader: {}", hasheader);
        println!("\tkey_fields: {:?}", key_fields);
        println!("\tunique_fields: {:?}", unique_fields);
        println!("\tfile list {:?}", filelist);
        if filelist.len() <= 0 {
            println!("\tprocessing stdin");
        }
    }

    let mut hm : BTreeMap<String, KeySum> = BTreeMap::new();
    let mut count = 0;

    let mut total_rowcount = 0usize;
    let mut total_fieldcount = 0usize;
    let mut total_bytes = 0usize;
    let start_f = Instant::now();

    if filelist.len() <= 0 {
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let (rowcount, fieldcount) = process(&mut handle, &mut hm, delimiter, & key_fields, & unique_fields);
        total_rowcount += rowcount;
        total_fieldcount += fieldcount;
    } else {
        for filename in filelist.into_iter() {
           let metadata = fs::metadata(&filename)?;
            // let metadata = match fs::metadata(&filename) {
            //     Ok(m) => m,
            //     Err(err) => return Err(std::error::Error::new(err.kind(), format!("could not get stats on file {}, cause: {}", &filename, err.description()) )),
            // );

            println!("file: {}", filename);
            let f = match OpenOptions::new()
                    .read(true)
                    .write(false)
                    .create(false)
                    .open(&filename)
                    {
                        Ok(f) => f,
                        Err(e) => panic!("cannot open file \"{}\" due to this error: {}",filename, e),
                    };
            let mut handle = BufReader::with_capacity(1024*1024*16,f);
            let (rowcount, fieldcount) = process(&mut handle, &mut hm, delimiter, & key_fields, & unique_fields);
            total_rowcount += rowcount;
            total_fieldcount += fieldcount;
            total_bytes += metadata.len() as usize;
        }
    }

    for (ff,cc) in &hm {
        println!("{} {}", ff,cc.count);
    }
    if ( verbose ) {
        let elapsed = start_f.elapsed();
        let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
        let rate : f64= (total_bytes as f64 / (1024f64*1024f64)) as f64 / sec;
        println!("rows: {}  fields: {}  rate: {:.2}MB/s", total_rowcount, total_fieldcount, rate);
    }
    Ok( () )
}


fn process( rdr: &mut BufRead, hm : &mut BTreeMap<String, KeySum>,
    delimiter: char, key_fields : & Vec<usize>, unique_fields: & Vec<usize>) -> (usize,usize) {

    let mut ss : String = String::with_capacity(256);

    let mut recrdr = csv::ReaderBuilder::new()
        .delimiter(delimiter as u8).has_headers(false).flexible(true)
        .from_reader(rdr);
    //println!("{:?}", &recrdr);
    let mut rowcount = 0usize;
    let mut fieldcount = 0usize;
    for result in recrdr.records() {
        //println!("here");
        let record : csv::StringRecord = result.unwrap();
        //println!("{} {}", &record[0], &record[1]);
        ss.clear();

        for kfi in key_fields {
            if *kfi != key_fields.len() {
                ss.push_str(&record[*kfi]);
                ss.push(delimiter);
            } else {
                ss.push_str(&record[*kfi])
            }
        }

        rowcount += 1;
        fieldcount += record.len();
        {
            let v = hm.entry(ss.clone()).or_insert(KeySum{ count : 0  /*, unique_values: BTreeSet::new()*/ });
            v.count = v.count +1;
        }

    }

    (rowcount, fieldcount)
}
