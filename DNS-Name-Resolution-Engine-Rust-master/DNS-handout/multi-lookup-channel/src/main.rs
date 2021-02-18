use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter, Write};
use std::path::Path;
use dns_lookup::lookup_host;
use std::thread;
use crossbeam::channel::bounded;
use std::sync::{Mutex, Arc};
use std::time::Duration;
// use std::any::type_name;
// fn type_of<T>(_: T) -> &'static str {
//     type_name::<T>()
// }

// this function takes in the reader file, channel transmitter which is the sender and returns a thread joinhandle.
fn get_hostname(reader: BufReader<File>, s: crossbeam::channel::Sender<String>) -> thread::JoinHandle<()> {
    // It spawns a thread
    thread::spawn(move || {
        //It iterates through each line
        for line in reader.lines() {
            // It sends the result through the transmitter and unraps the result
            //println!(" -- I am in sender -- ");
            //thread::sleep(Duration::from_millis(10));
            s.send(line.unwrap()).unwrap();
        }
    })
}
// this function takes in the file and shared_rx which is a mutex for our receiver. We have actually wrapped out receiver with a mutex and an arc
fn display_ipaddress(of_file: Arc<Mutex<BufWriter<File>>>, shared_rx: Arc<crossbeam::channel::Receiver<String>>) {
    // We spawn another set of threads and we lopp them over and over again until we receive everithing that is transmitted through the sender
    let mut count = 0;
    thread::spawn(move || loop {
        count = count + 1;
        //println!("{} --------- I am in resolver -----------", count);
        let hostname = shared_rx.recv().unwrap();
        //while !hostname.is_empty() {
            let mut ips: Vec<std::net::IpAddr> = vec![];
            let get_ips = lookup_host(&hostname);
            match get_ips {
                Ok(v) => ips = v,
                Err(_e) => println!("dnslookup error: {:?}", &hostname),
            }
            let mut temp: String = "".to_string();
            if ips.len() > 0 {
            //Extra-credit
            for ip in ips {
                if !temp.is_empty() {
                    temp.push_str(", ");
                }
                temp.push_str(&ip.to_string());
            }
            // End - Extra-credit
            //   temp = ips[0].to_string();
            }
            
            let of_file = Arc::clone(&of_file);
            {
                let mut of_file = of_file.lock().unwrap();
                of_file.write_all(format!("{} {}\n",  hostname, temp).as_bytes()).ok();
                of_file.flush();
                println!("{}, {} {}", hostname, temp, hostname.len());
            }
        //}
    });
}
fn main() {
    let _max_input_files = 10;
    let _min_resolver_threads = 2;
    let _max_name_length = 1025;
    let _max_ip_length = 10;

    extern crate num_cpus;
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
    let of = BufWriter::new(ofile);
    let (s, r) = bounded(10);
    let shared_rx = Arc::new(r);
    let of_file = Arc::new(Mutex::new(of));
    //let mut threads = vec![];
    
    // Extra Credit
    while _max_resolver_threads > 0 {
        display_ipaddress(of_file.clone(), shared_rx.clone());
        _max_resolver_threads = _max_resolver_threads - 1;
    }
    // Extra Credit 
    
    //display_ipaddress(of_file.clone(), shared_rx.clone());
    //display_ipaddress(of_file.clone(), shared_rx.clone());

    for input in inputs {
        let path = Path::new(&input);
        let display = path.display();
        let file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        let reader = BufReader::new(file);
        get_hostname(reader, s.clone()).join().unwrap();
    }
}