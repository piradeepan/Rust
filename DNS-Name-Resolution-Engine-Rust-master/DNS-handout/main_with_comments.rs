use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter, Write};
use std::path::Path;
use dns_lookup::lookup_host;

fn main() {
    // documentation on how to accept command line arguments
    //https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html

    // Vector of strings that collects all the arguments from the environment
    let args: Vec<String> = env::args().collect();
    // This allows to get a slice for just the output file. Just grab the last thing and put it into outfile
    let outfile = &args[args.len()-1];
    // Get the path of the outfile
    let path = Path::new(&outfile);
    // To print the path using display
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    // the below creates a output file form the path
    let ofile = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(ofile) => ofile,
    };
    // make a Buffered writer from the output file
    let mut of = BufWriter::new(ofile);
    // slice of the array i.e. from the vector of args and put it into a new vector
    let inputs: Vec<String> = args[1..args.len()-1].to_vec();
    // Iterate through the inputs and pop each string out of the input string 
    for input in inputs {
        // Create a path to the desired file
        // get path for each inout
        let path = Path::new(&input);
        // print out the display to see what the path was
        let display = path.display();
        // Open the path in read-only mode, returns `io::Result<File>`
        // match the Err or the Ok
        let file = match File::open(&path) {
            // If the creation fails, it panics and says why it failed
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        // Similarly BufferedReader vs BufferedWriter
        let reader = BufReader::new(file);
        // For each line in the file
        for line in reader.lines() {
            // unwrap that line and put it into the hostname
            // unwrap is because the line is going to have a result
            let hostname = line.unwrap();
            // get the vector of ip's
            // mutable vector of ip addresses that is getting a blank vector
            let mut ips: Vec<std::net::IpAddr> = vec![];
            let get_ips = lookup_host(&hostname);
            // match the ips and print it as below correspondingly
            match get_ips {
                Ok(v) => ips = v,
                Err(_e) => eprintln!("dnslookup error: {:?}", hostname),
            }
            let mut temp: String = "".to_string();
            if ips.len() > 0 {
                temp = ips[0].to_string();
            }
            // write function to write it to the output file
            of.write(format!("{} {}\n",  hostname, temp).as_bytes()).ok();
        }
    }
}



