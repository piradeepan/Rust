use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter, Write};
use std::path::Path;
use dns_lookup::{lookup_host};
use crossbeam::queue::ArrayQueue;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};


fn main() {
    //https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
    let args: Vec<String> = env::args().collect();
    let outfile = &args[args.len()-1];
    let path = Path::new(&outfile);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let ofile = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(ofile) => ofile,
    };

    let of = Arc::new(Mutex::new(BufWriter::new(ofile)));
    let inputs: Vec<String> = args[1..args.len()-1].to_vec();
    let q = Arc::new(ArrayQueue::new(10));
    let mut handle = vec![];    
    let q1 = Arc::clone(&q);
    let flag = Arc::new(Mutex::new(true));
    let f1 = Arc::clone(&flag);


    handle.push(thread::spawn(move || {
        for input in inputs {
            // Create a path to the desired file
            let path = Path::new(&input);
            let display = path.display();
            // Open the path in read-only mode, returns `io::Result<File>`
            let file = match File::open(&path) {
                Err(why) => panic!("couldn't open {}: {}", display, why),
                Ok(file) => file,
            };
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let hostname = line.unwrap();
                let host = hostname.clone();
                q1.push(String::from(host));
            }
        }
        *f1.lock().unwrap() = false;
    }));

//First resolver
    let q1 = Arc::clone(&q);
    let of1 = Arc::clone(&of);
    let f2 = Arc::clone(&flag);
    handle.push(thread::spawn(move || loop {
        match q1.pop(){
            None => {
                //*f2.lock().unwrap() = true;
                thread::sleep(Duration::from_millis(100));
            }
            Some(hostname) => {
                println!("{:?}", hostname);
                let mut ips: Vec<std::net::IpAddr> = vec![];
                let version = lookup_host(&hostname);
                match version {
                    Ok(v) => ips = v,
                    Err(_e) => eprintln!("dnslookup error: {:?}", hostname),
                }
                let mut temp: String = "".to_string();
                if ips.len() > 0 {
                    temp = ips[0].to_string();
                }
                {
                    let mut of3 = of1.lock().unwrap();
                    of3.write(format!("{} {}\n",  hostname, temp).as_bytes());
                }
                if q1.is_empty() && *f2.lock().unwrap() == false {
                    break;
                }
            }
        }
    }));

//Second resolver
    let q2 = Arc::clone(&q);
    let of2 = Arc::clone(&of);
    let f3 = Arc::clone(&flag);
    handle.push(thread::spawn(move || loop {
        match q2.pop(){
            None => {
                //*f3.lock().unwrap() = true;
                thread::sleep(Duration::from_millis(100));
            }
            Some(hostname) => {
                println!("{:?}", hostname);
                let mut ips: Vec<std::net::IpAddr> = vec![];
                let version = lookup_host(&hostname);
                match version {
                    Ok(v) => ips = v,
                    Err(_e) => eprintln!("dnslookup error: {:?}", hostname),
                }
                let mut temp: String = "".to_string();
                if ips.len() > 0 {
                    temp = ips[0].to_string();
                }
                {
                    let mut of4 = of2.lock().unwrap();
                    of4.write(format!("{} {}\n",  hostname, temp).as_bytes());
                }

                if q2.is_empty() && *f3.lock().unwrap() == false {
                    break;
                }
            }
        }

    }));
    for combine in handle {
        combine.join().unwrap();
    }
}



