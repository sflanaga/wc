extern crate flate2;
extern crate tar;
extern crate csv;

use std::fs::File;
use std::fs::OpenOptions;
use flate2::read::GzDecoder;
use tar::Archive;
use std::env::args;
use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;
use std::collections::BTreeMap;

fn walk_tar<R>(af: &mut Archive<R>, delimiter: char, header: bool, writers: &mut BTreeMap<u32, BufWriter<File>>, slots: u32)
    where R: Read
{
    /*let mut aa: Archive<Read> = Archive::new(af); */
    for file in af.entries().unwrap() {
        // Make sure there wasn't an I/O error
        let file = file.unwrap();
        let path = String::from(file.header().path().unwrap().to_str().unwrap());
        let sz: u64 = file.header().size().unwrap();
        let mut count = 0;
        {
            let filbuf = BufReader::new(file);
            let mut recrdr = csv::ReaderBuilder::new()
                .delimiter(delimiter as u8).has_headers(header).flexible(true)
                .from_reader(filbuf);
            let mut strbuf = String::with_capacity(256);

            for result in recrdr.records() {

                let record : csv::StringRecord = result.unwrap();
                let n = &record[2].parse::<u32>().unwrap();

                let slot = n % slots;

                {
                    let w = writers.entry(slot).or_insert_with(|| {
                        let filename = format!("outfile_{}.csv", slot);
                        let f = match OpenOptions::new()
                                .read(false)
                                .write(true)
                                .create(true)
                                .truncate(true)
                                .open(&filename)
                                {
                                    Ok(f) => f,
                                    Err(e) => panic!("cannot open file \"{}\" due to this error: {}",&filename, e),
                                };
                        BufWriter::with_capacity(1024*1024,f)

                    });
                    strbuf.clear();
                    for s in &record {
                        strbuf.push_str(s);
                        strbuf.push_str("|");
                    }
                    if strbuf.len() > 0 {
                        strbuf.pop();
                    }
                    write!(w, "{}\n", &strbuf );
                }




            }

        }
        // Inspect metadata about the file
        println!("{}  lines: {}  size: {}", path, count, sz);
    }

}

fn run() {

    // this should probably just be a vector but the difference of writing a single file
    // or X files is not measurable
    let mut writers : BTreeMap<u32, BufWriter<File>> = BTreeMap::new();
    let slots = args().nth(1).expect("missing 1st arg for number of split slots").parse::<u32>().unwrap();
    let path = args().nth(2).expect("missing 2nd arg for input tar(.gz) file");
    println!("looking at {}", path);
    if path.ends_with(".gz") {
        let mut aa = Archive::new(GzDecoder::new(File::open(path.clone()).unwrap()).unwrap());
        walk_tar(&mut aa, ',', false, &mut writers, slots);
    } else {
        let mut aa = Archive::new(File::open(path.clone()).unwrap());
        walk_tar(&mut aa, ',', false, &mut writers, slots);
    };

}


fn main() {
    run();
}
