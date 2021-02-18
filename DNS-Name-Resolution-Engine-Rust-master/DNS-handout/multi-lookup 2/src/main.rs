use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter, Write};
use std::path::Path;
use dns_lookup::lookup_host;
use std::thread;
use std::time::Duration;
use crossbeam_channel::bounded;
use std::sync::{Mutex, Arc};

fn get_hostname(inputs: &Vec<String>, s: crossbeam_channel::Sender<String>) {
    for input in inputs {
        let path = Path::new(&input);
        let display = path.display();

        let file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        let reader = BufReader::new(file);
        let sender = s.clone();
        thread::spawn(move || {
            for line in reader.lines() {
                let hostname = line.unwrap();
                sender.send(hostname).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
            drop(sender);
        });
    }
}
fn display_ipaddress(of_file: Arc<Mutex<BufWriter<File>>>, r: crossbeam_channel::Receiver<String>) {
    let receiver = r.clone();
    let mut threads = vec![];
    for host_result in receiver {
        let mut ips: Vec<std::net::IpAddr> = vec![];
        let get_ips = lookup_host(&host_result);
        match get_ips {
            Ok(v) => ips = v,
            Err(_e) => eprintln!("dnslookup error: {:?}", host_result),
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
        threads.push(thread::spawn(move || {
            {
                let mut of_file = of_file.lock().unwrap();
                of_file.write(format!("{} {}\n",  host_result, temp).as_bytes()).ok();
            }
        }));
    }
}
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
    let of_file = Arc::new(Mutex::new(of));
    //let rx = Arc::new(Mutex::new(0));

    let (s, r) = bounded(10);
    //let shared_r = Arc::new(r);
    //let (_s1, r1) = (s.clone(), rx.clone());

    get_hostname(&inputs, s);
    display_ipaddress(of_file.clone(), r);
    //display_ipaddress(&mut of, r1);
}