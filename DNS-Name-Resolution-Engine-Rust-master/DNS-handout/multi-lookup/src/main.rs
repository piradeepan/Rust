use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter, Write};
use std::path::Path;
use dns_lookup::lookup_host;
use std::thread;
use std::time::Duration;
use crossbeam::queue::ArrayQueue;
use std::sync::{Mutex, Arc};

//use std::any::type_name;

fn main() {
    let _max_input_files = 10;
    let _min_resolver_threads = 2;
    let _max_name_length = 1025;
    let _max_ip_length = 10;

    extern crate num_cpus;
    let _max_resolver_threads = num_cpus::get();
    //println!("Number of logical cores is {}", _max_resolver_threads);

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
    let of = BufWriter::new(ofile);
    let q2 = ArrayQueue::new(10);
    let q = Arc::new(q2);
    let shared_q = q.clone();
    let mut threads = vec![];
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
                let hostname = line.unwrap();
                q.push(hostname);
                thread::sleep(Duration::from_millis(100));
            }
        }
    }));
    threads.push(thread::spawn(move || {
        let of_file = Arc::new(Mutex::new(of));
        while !shared_q.is_empty() {
            match shared_q.pop() {
                None => {
                    thread::sleep(Duration::from_millis(10));
                }
                Some(hostname) => {
                    let mut ips: Vec<std::net::IpAddr> = vec![];
                    let get_ips = lookup_host(&hostname);
                    match get_ips {
                        Ok(v) => ips = v,
                        Err(_e) => println!("dnslookup error: {:?}", &hostname),
                    }
                    let mut temp: String = "".to_string();
                    if ips.len() > 0 {
                        /* Extra-credit
                        for ip in ips {
                            if !temp.is_empty() {
                                temp.push_str(", ");
                            }
                            temp.push_str(&ip.to_string());
                        }
                        // End - Extra-credit */
                        temp = ips[0].to_string();
                    }
                    let of_file = Arc::clone(&of_file);
                    {
                        let mut of_file = of_file.lock().unwrap();
                        of_file.write(format!("{} {}\n",  hostname, temp).as_bytes()).ok();
                    }
                }
            }
        }
    }));
}