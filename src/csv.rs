#![allow(unused_imports)]
extern crate csv;
extern crate prettytable;

use std::io::prelude::*;
use std::fs::OpenOptions;
use std::fs;
use std::process;
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
use std::collections::HashSet;

use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use prettytable::format;

mod gen;

use gen::greek;

#[derive(Debug)]
struct KeySum {
    count : u64,
    sums : Vec<f64>,
    distinct: Vec<HashSet<String>>
}
fn main() {
    if let Err(err) = csv() {
        println!("error: {:?}", &err);
        std::process::exit(1);
    }
}

fn help(msg: &str) {
    println!("error: {}", msg);
println!(r###"csv [options] file1... fileN
csv [options] -i # read from standard input
    --help this help
    -h - data has header so skip first line
    # All the follow field lists are zero based - first field is 0
    -f x,y...z - comma seperated field list of fields to use as a group by
    -s x,y...z - comma seperated field list of files to do a f64 sum of
    -u x,y...z - comma seperated field list of unique or distinct records
    -a auto alignment or table format OFF - use csv format
    -d input_delimiter - single char
    -v - verbose
    -i read from standard input
    --nc - do not write record counts
"###);
process::exit(1);
}

fn csv() -> Result<(),std::io::Error> {

    let mut key_fields = vec![];
    let mut unique_fields = vec![];
    let mut sum_fields = vec![];
    let mut delimiter : char = ',';
//    let mut output_delimiter: char = ',';
    let mut auto_align: bool = true;
    let mut verbose = false;
    let mut hasheader = false;
    let mut write_record_count = true;

    let argv : Vec<String> = args().skip(1).map( |x| x).collect();
    let filelist = &mut vec![];

    let mut i = 0;

    if argv.len() <= 1 {
        help("no command line options used");
    }

    while i < argv.len() {
        match &argv[i][..] {
            "--help" => { // field list processing
                help("command line requested help info");
            },
            "-a" => {
                auto_align = false;
            },
            "-f" => { // field list processing
                i += 1;
                key_fields.splice(.., (&argv[i][..]).split(",").map( |x| x.parse::<usize>().unwrap()) );
            },
            "-s" => { // field list processing
                i += 1;
                sum_fields.splice(.., (&argv[i][..]).split(",").map( |x| x.parse::<usize>().unwrap()) );
            },
            "-u" => { // unique count AsMut
                i += 1;
                unique_fields.splice(.., (&argv[i][..]).split(",").map( |x| x.parse::<usize>().unwrap()) );
            },
            // "--od" => { // unique count AsMut
            //     i += 1;
            //     output_delimiter = argv[i].as_bytes()[0] as char;
            // },
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
            "--nc" => { // just write the keys and not the row count
                write_record_count = false;
            },
            x => {
                if verbose { println!("adding filename {} to scan", x); }
                filelist.push(x);
            }
        }

        i += 1;
    }
    // if key_fields.len() <= 0 {
    //     help("missing key fields - you must specify -f option with something or no summaries can be made");
    // }

    let maxfield = 1;

    if verbose {
        println!("\tdelimiter: {}", delimiter);
        println!("\theader: {}", hasheader);
        println!("\tkey_fields: {:?}  len={}", key_fields, key_fields.len() );
        println!("\tsum_fields: {:?}  len={}", sum_fields, sum_fields.len() );
        println!("\tunique_fields: {:?}", unique_fields);
        println!("\tfile list {:?}", filelist);
        if filelist.len() <= 0 {
            println!("\tprocessing stdin");
        }
    }

    let mut hm : BTreeMap<String, KeySum> = BTreeMap::new();

    let mut total_rowcount = 0usize;
    let mut total_fieldcount = 0usize;
    let mut total_bytes = 0usize;
    let start_f = Instant::now();

    if filelist.len() <= 0 {
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let (rowcount, fieldcount) = process(&mut handle, &mut hm, delimiter, & key_fields, & sum_fields, & unique_fields, hasheader, verbose);
        total_rowcount += rowcount;
        total_fieldcount += fieldcount;
    } else {
        for filename in filelist.into_iter() {
           // let metadata = fs::metadata(&filename)?;
            let metadata = match fs::metadata(&filename) {
                Ok(m) => m,
                Err(err) => {
                    eprintln!("skipping file, could not get stats on file {}, cause: {}", &filename, err);
                    continue;
                },
            };

            if verbose { println!("file: {}", filename); }
            let f = match OpenOptions::new()
                    .read(true)
                    .write(false)
                    .create(false)
                    .open(&filename)
                    {
                        Ok(f) => f,
                        Err(e) => panic!("cannot open file \"{}\" due to this error: {}",filename, e),
                    };
            let mut handle = BufReader::with_capacity(1024*1024*4,f);
            let (rowcount, fieldcount) = process(&mut handle, &mut hm, delimiter, & key_fields, & sum_fields, & unique_fields, hasheader, verbose);
            total_rowcount += rowcount;
            total_fieldcount += fieldcount;
            total_bytes += metadata.len() as usize;
        }
    }

    // if auto_align {

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        for (ff,cc) in &hm {
            let mut vcell = vec![];
            let z1: Vec<&str> = ff.split('|').collect();
            for x in &z1 {
                vcell.  push(Cell::new(x));
            }
            // z1.iter().map( |x| { println!("{}", x); vcell.push(Cell::new(x));} );
            // //vcell.push(Cell::new(&ff));
            if write_record_count {
                vcell.push(Cell::new(&format!("{}",cc.count)));
            }
            for x in &cc.sums {
                vcell.push(Cell::new(&format!("{}",x)));
            }
            for x in &cc.distinct {
                vcell.push(Cell::new(&format!("{}",x.len())));
            }
            let mut row = Row::new(vcell);
            table.add_row(row);
        }
        if auto_align {
            table.printstd();
        } else {
            println!("{}", table.to_csv(Vec::new()).unwrap().into_string());
        }

    // } else {
    //     let mut tmp_s = String::new();
    //     for (ff,cc) in &hm {
    //         tmp_s.truncate(0);
    //         tmp_s.push_str(&format!("{}",&ff));
    //         if write_record_count {
    //             tmp_s.push_str(&format!("{}{}",output_delimiter, cc.count));
    //         }
    //         for x in &cc.sums {
    //             tmp_s.push_str(&format!("{}{}", output_delimiter, x));
    //         }
    //         for x in &cc.distinct {
    //             tmp_s.push_str(&format!("{}{}", output_delimiter, x.len()));
    //         }
    //         println!("{}", tmp_s);
    //     }
    // }

    if ( verbose ) {
        let elapsed = start_f.elapsed();
        let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
        let rate : f64= (total_bytes as f64 / (1024f64*1024f64)) as f64 / sec;
        if verbose {
            println!("rows: {}  fields: {}  rate: {:.2}MB/s", total_rowcount, total_fieldcount, rate);
        }
    }
    Ok( () )
}


fn process( rdr: &mut BufRead, hm : &mut BTreeMap<String, KeySum>,
    delimiter: char, key_fields : & Vec<usize>, sum_fields : & Vec<usize>, unique_fields: & Vec<usize>, header: bool, verbose: bool) -> (usize,usize) {

    let mut ss : String = String::with_capacity(256);

    let mut recrdr = csv::ReaderBuilder::new()
        .delimiter(delimiter as u8).has_headers(header).flexible(true)
        .from_reader(rdr);
    //println!("{:?}", &recrdr);
    let mut rowcount = 0usize;
    let mut fieldcount = 0usize;
    let mut sum_grab = vec![];
    let mut uni_grab = vec![];

    for result in recrdr.records() {
        //println!("here");
        //

        let record : csv::StringRecord = result.unwrap();
        //println!("{} {}", &record[0], &record[1]);
        ss.clear();

        let mut i = 0;
        if key_fields.len() > 0 {
            while i < key_fields.len() {
                let index = key_fields[i];
                if index < record.len() {
                    ss.push_str(&record[index]);
                } else {
                    ss.push_str("NULL");
                }
                if i != key_fields.len()-1 {
                    ss.push('|');
                }
                i += 1;
            }
        } else {
            ss.push_str("NULL");
        }

        if sum_fields.len() > 0 {
            sum_grab.truncate(0);
            i=0;
            while i < sum_fields.len() {
                let index = sum_fields[i];
                if index < record.len() {
                    let v = &record[index];
                    match v.parse::<f64>() {
                        Err(_) => {
                            if verbose {
                                println!("error parseing string |{}| as a float for summary index: {} so pretending value is 0",v, index);
                            }
                            sum_grab.push(0f64);},
                        Ok(vv) => sum_grab.push(vv),
                    }
                } else {
                    sum_grab.push(0f64);
                }
                i += 1;
            }
        }

        if unique_fields.len() > 0 {
            uni_grab.truncate(0);
            i=0;
            while i < unique_fields.len() {
                let index = unique_fields[i];
                if index < record.len() {
                    uni_grab.push(record[index].to_string());
                } else {
                    uni_grab.push("NULL".to_string());
                }
                i += 1;
            }
        }

        if ss.len() > 0 {
            rowcount += 1;
            fieldcount += record.len();
            {
                let v = hm.entry(ss.clone()).or_insert(KeySum{ count: 0, sums: sum_grab.to_vec(), distinct: Vec::new() });
                v.count = v.count +1;
                for (i,f) in sum_grab.iter().enumerate() {
                    v.sums[i] = v.sums[i] + f;
                }
                if uni_grab.len() > 0 {
                    while v.distinct.len() < uni_grab.len() {
                        v.distinct.push(HashSet::new());
                    }
                    for (i,u) in uni_grab.iter().enumerate() {
                        v.distinct[i].insert(u.to_string());
                    }
                }
            }
        }

    }

    (rowcount, fieldcount)
}
