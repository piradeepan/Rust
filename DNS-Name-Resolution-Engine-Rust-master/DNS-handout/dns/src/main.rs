// Piradeepan Nagarajan, Subhed Chavan
// CSCI640
// Assignment 5
// Submitted on 11/27/2020

use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter, Write};
use std::path::Path;
use dns_lookup::lookup_host;
use std::thread;
use crossbeam::channel::bounded;
use std::sync::{Mutex, Arc};

fn main() {
    let _max_input_files = 10;
    let _min_resolver_threads = 2;
    let _max_name_length = 1025;
    let _max_ip_length = 10;

    let mut _max_resolver_threads = num_cpus::get();

    let args: Vec<String> = env::args().collect();
    let outfile = &args[args.len()-1];
    let path = Path::new(&outfile);
    let display = path.display();

    let inputs: Vec<String> = args[1..args.len()-1].to_vec();
    if inputs.len() > _max_input_files {
		println!("Input files not allowed more than 10");
		return;
	}
    let ofile = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(ofile) => ofile,
    };
    //Wraps a writer and buffers its output.
    let of = BufWriter::new(ofile);
    let (s, r) = bounded(10);
    // Giving a shared wonership for the receiver
    let shared_rx = Arc::new(r);
    let of_file = Arc::new(Mutex::new(of));
    let mut threads = vec![];
    let s1 = s.clone();
    let flag = Arc::new(Mutex::new(false));
    let flag1 = flag.clone();
    threads.push(thread::spawn(move || {
        for input in inputs {
            let path = Path::new(&input);
            let display = path.display();
            let file = match File::open(&path) {
                Err(why) => panic!("couldn't open {}: {}", display, why),
                Ok(file) => file,
            };
            let reader = BufReader::new(file);
            for line in reader.lines() {
                s1.send(line.unwrap()).unwrap();
                
            }
        }
        *flag1.lock().unwrap() = true;
    }));
    for _ in 0..2 {
        let shared_rx = shared_rx.clone();
        let s2 = s.clone();
        let of_file = of_file.clone();
        let flag2 = flag.clone();
        threads.push(thread::spawn(move || loop {
            let mut temp: String = "".to_string();
            let mut hostname: String = "".to_string();
            match shared_rx.recv() {
                Ok(_received) => {
                    hostname = _received;
                }, Err(_) => ()   
            }
            let mut ips: Vec<std::net::IpAddr> = vec![];
            let get_ips = lookup_host(&hostname);
            match get_ips {
                Ok(v) => ips = v,
                Err(_e) => println!("dnslookup error: {:?}", &hostname),
            }
            if ips.len() > 0 {
                temp = ips[0].to_string();
            }
            {
                let mut of_file = of_file.lock().unwrap();
                of_file.write(format!("{} {}\n",  hostname, temp).as_bytes()).ok();
                of_file.flush().ok();
            }
            if shared_rx.is_empty() && s2.is_empty() && *flag2.lock().unwrap() {
                break;
            }
        }));
    }
    for child in threads {
        child.join().unwrap();
    }
}