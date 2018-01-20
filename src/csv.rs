#![allow(unused_imports)]


use std::io::prelude::*;
use std::fs::OpenOptions;

use std::string::String;
use std::env::args;
use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;
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
    unique_values: BTreeSet<String>,
    count : u64
}


fn main() {

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
        println!("delimiter: {}", delimiter);
        println!("header: {}", hasheader);
        println!("key_fields: {:?}", key_fields);
        println!("unique_fields: {:?}", unique_fields);
        println!("file list {:?}", filelist);
        if filelist.len() > 0 {
            for f in filelist.into_iter() {
                println!("file: {}", f);
            }
        } else {
            println!("processing stdin");
        }
    }

    let mut hm : BTreeMap<String, KeySum> = BTreeMap::new();
    let mut count = 0;

    if filelist.len() <= 0 {
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let (rowcount, fieldcount) = process(&mut handle, &mut hm, delimiter, & key_fields, & unique_fields);
        println!("fieldcount {}", rowcount);
    }
    // for (ff,cc) in &hm {
    //     println!("{} <=> {}  {}", ff,cc.count, cc.unique_values.len());
    // }

}


fn process( rdr: &mut BufRead, hm : &mut BTreeMap<String, KeySum>,
    delimiter: char, key_fields : & Vec<usize>, unique_fields: & Vec<usize>) -> (usize,usize) {

    let mut ss : String = String::with_capacity(256);


    let mut recrdr = csv::ReaderBuilder::new()
        .delimiter(delimiter as u8).has_headers(false)
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
            let v = hm.entry(ss.clone()).or_insert(KeySum{ count : 0, unique_values: BTreeSet::new() });
            v.count = v.count +1;
        }

    }

    (rowcount, fieldcount)
}
