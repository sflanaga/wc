#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};

use std::env::args;
use std::time::Instant;

mod wc;
mod memh;

use memh::map_test;

use wc::mainfile;
use wc::mainstdin;

use std::io::Read;


fn main() {
    println!("  +  {}", args().nth(0).unwrap());

    let matches = App::new("tester")
        .version("1.0")
        .author("Steve F<disposesmf@gmail.com>")
        .about("test things")
        .subcommand(
            SubCommand::with_name("wc")
                .about("line couter")
                .version("0.1")
                .author("Someone E. <someone_else@other.com>")
                //.arg_from_usage("-d, --debug 'Print debug information'"),
                .arg_from_usage("[input]... 'an optional input file to use'")
                .arg(Arg::with_name("slow")
                    .takes_value(false)
                    .short("slow")
                    .help("use slow path for line counting")
                )
                
        )
        .subcommand(
            SubCommand::with_name("printbubba")
                .about("line couter")
        )
        .subcommand(
            SubCommand::with_name("memh")
                .about("memory test command")
                .arg(Arg::with_name("iterations")
                    .index(1)
                    .required(true)
                    .help("number of iterations to execute")
                )
                .arg(Arg::with_name("pauses")
                    .takes_value(false)
                    .short("p")
                    .help("pause with std in")
                )
                .arg(Arg::with_name("capacity")
                    .takes_value(false)
                    .short("c")
                    .help("pre allocate hash size using with_capacity")
                )
                .arg(Arg::with_name("tree")
                    .takes_value(false)
                    .short("t")
                    .help("use BTreeMap instead of default HashMap")
                )
                .arg(Arg::with_name("u64")
                    .takes_value(false)
                    .long("u64")
                    .help("use u64 int values in map instead of u32")
                )
        )
        .get_matches();


    let now = Instant::now();
        

    if let Some(matches) = matches.subcommand_matches("memh") {
            let iterations = value_t!(matches.value_of("iterations"), usize).unwrap_or_else(|e| e.exit());
            let pause = matches.is_present("pauses");
            let tree = matches.is_present("tree");
            let capacity = matches.is_present("capacity");
            println!("pausing so hit retrun at key points");
            match matches.is_present("u64") {
                false => { println!("u32");  map_test::<u32>(pause, iterations, capacity, tree) },
                true => { println!("u64");   map_test::<u64>(pause, iterations, capacity, tree) },
            }
            
            if pause {
                let mut buf: [u8; 1] = [0; 1];
                let stdin = ::std::io::stdin();
                let mut stdin = stdin.lock();
                let _it = stdin.read(&mut buf[..]);
            }


    } else if let Some(_matches) = matches.subcommand_matches("printbubba") {
            println!("printing bubba shrubbery");
    } else if let Some(matches) = matches.subcommand_matches("wc") {
        println!("sub wc...");
        let slow = matches.is_present("slow");
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
                mainfile(inp_files,slow);
        } else {
                mainstdin(slow);
        }
        
        if matches.is_present("debug") {
            println!("wc with debug");
        } else {
            println!("wc no debug");
        }
    }
        let elapsed = now.elapsed();
        let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
        println!("{} seconds ", sec);

}


