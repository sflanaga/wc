// #![feature(libc)]

#![allow(unused_imports)]


use std::any::Any;

use std::io::stdin;
use std::io::Write;
use std::io::Read;
use std::io::{self, BufReader};
use std::hash::Hash;
use std::cmp::Ord;

extern crate rand;
extern crate heapsize;

use std::collections::HashMap;
use std::collections::BTreeMap;

//use rand;
use self::rand::Rng;
use self::rand::Rand;


extern crate libc;
use self::libc::*;

use std::time::{Instant, Duration};

use std::thread;

use std::sync::{RwLock, mpsc, Arc};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

/*
extern {fn je_stats_print (write_cb: extern fn (*const c_void, *const c_char), cbopaque: *const c_void, opts: *const char);}
extern fn write_cb (_: *const c_void, message: *const c_char) {
    print! ("{}", String::from_utf8_lossy (unsafe {std::ffi::CStr::from_ptr (message as *const i8) .to_bytes()}));
}

fn stats_print() {
    unsafe {je_stats_print (write_cb, std::ptr::null(), std::ptr::null())};
}
*/

static LEN: AtomicUsize = ATOMIC_USIZE_INIT;
static ADDS : AtomicUsize = ATOMIC_USIZE_INIT;

fn tickerthread(rx: Receiver<i32>) {
    const SLEEPTIME: u64 = 1000;
    let mut last_adds: usize = 0;
    loop {
        //              thread::sleep(time::Duration::from_millis(1000));
        let rci = match rx.recv_timeout(Duration::from_millis(SLEEPTIME)) {
            Ok(i) => i,
            Err(_) => {
                /* println!("time or disconnect but we ignore {}", e); */
                -1
            }
        };
        if rci == 5 {
            println!("\rticker thread exiting");
            return;
        }
        // let bytesread = BYTES.fetch_add(0, Ordering::SeqCst) as f64;
        // let mb_rate = (bytesread - lastbytesread) * (1000.0 / SLEEPTIME as f64);

        let curr_len = LEN.fetch_add(0, Ordering::SeqCst);
        let curr_adds = ADDS.fetch_add(0, Ordering::SeqCst);
        let add_rate = ( (curr_adds-last_adds) as f64) * (1000.0 / SLEEPTIME as f64);
        println!("Progress:  adds: {}  len: {}  add_rate: {}/s",
               curr_adds,
               curr_len,
               add_rate
               );
        io::stdout().flush().unwrap();

        last_adds = curr_adds;

    }
}




/* one attempt a extend poly:

https://play.rust-lang.org/?gist=a043a576a050b8a221e1a919ee0dd341&version=undefined

*/


pub fn map_test<T>(pause: bool, iterations: usize, precapacity: bool, tree: bool)  -> ()
where T : Hash+Ord+Rand 
{
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    let child = thread::Builder::new()
        .name("ticker".to_string())
        .spawn(|| tickerthread(rx))
        .unwrap();

    let now = Instant::now();

    if tree {
        let mut map: BTreeMap<T,T> = BTreeMap::new();

        let mut rng = rand::thread_rng();
        for x in 0..iterations {
            map.insert(rng.gen::<T>(), rng.gen::<T>());
            if x % 100 == 0 {
                ADDS.fetch_add(100, Ordering::SeqCst);
                LEN.store(map.len(), Ordering::SeqCst);
            }
        }
        let elapsed = now.elapsed();
        let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
        println!("complete {} seconds ", sec);
        println!("did {} iterations with a ending size of {} ", iterations, map.len());
    } else {
        let mut map: HashMap<T,T> = if precapacity {
            HashMap::with_capacity(iterations)
        } else {
            HashMap::new()
        };

        let mut rng = rand::thread_rng();
        for x in 0..iterations {
            map.insert(rng.gen::<T>(), rng.gen::<T>());
            if x % 100 == 0 {
                ADDS.fetch_add(100, Ordering::SeqCst);
                LEN.store(map.len(), Ordering::SeqCst);
            }
        }
        let elapsed = now.elapsed();
        let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
        println!("complete {} seconds ", sec);
        println!("did {} iterations with a ending size of {} ", iterations, map.len());
    }
    
    match tx.send(5) {
        Err(e) => println!("error on send call {}", e),
        Ok(_) => {}
    }

    let _res = child.join().unwrap();

    if pause {
        let mut buf: [u8; 1] = [0; 1];
        let stdin = ::std::io::stdin();
        let mut stdin = stdin.lock();
        let _it = stdin.read(&mut buf[..]);
    }
}
