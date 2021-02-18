use std::env;
use std::str;
use std::thread;
use std::io::{Read, Write};
use std::net::{TcpListener};
use std::sync::{Mutex, Arc};
use chrono::{Utc};
use std::fs::OpenOptions;
struct UriParts {
    method: String,
    uri: String,
    hostname: String,
    pathname: String,
    version: String,
    port: u16,
}
fn parse_uri(request: String) -> UriParts {
    let parts: Vec<&str> = request.split(' ').collect();
    let uri_parts: Vec<&str> = parts[1].splitn(4, '/').collect();
    
    let mut pathname: String = "".to_string();
    if uri_parts.len() > 3 {
        pathname = uri_parts[3].to_string()
    }
    let mut port: u16 = 80;
    let mut hostname = uri_parts[2];
    if uri_parts[2].contains(":") {
        let host_parts: Vec<&str> = uri_parts[2].split(":").collect();
        hostname = host_parts[0];
        port = host_parts[1].parse::<u16>().unwrap();
    }

    let uri_parts_struct = UriParts {
        method: parts[0].to_string(),
        version: parts[2].to_string(),
        uri: parts[1].to_string(),
        hostname: hostname.to_string(),
        pathname: pathname,
        port: port,
    };
    return uri_parts_struct
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // get the port numebr
    let input = &args[args.len()-1];
    println!("{}", &args[args.len()-2]);
    //A 64-bit floating point type (specifically, the "binary64" type defined in IEEE 754-2008).
    if !input.parse::<f64>().is_ok() {
        println!("Usage: cargo run <port number>");
        return;
    }
    let of = OpenOptions::new().append(true).write(true).create(true).open("proxy.log").expect("Cannot open file");
    let of_file = Arc::new(Mutex::new(of));
    //The 32-bit unsigned integer type.
    let port: u32  = input.parse().unwrap();
    let host_port= format!("{}:{}","0.0.0.0", port);
    // To listen for incoming connections you call the static method. passing it an address and a port, like so creates a TcpListener instance and binds it to the given address and port.
    let listener = TcpListener::bind(host_port).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let of_file = of_file.clone();
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    let now = Utc::now();
                    // u8 is the 8-bit unsigned integer type
                    let mut input = [0 as u8;100];
                    let _ = stream.read(&mut input);
                    let mut f = String::new();
                    //Converts a slice of bytes to a string, including invalid characters
                    f.push_str(&String::from_utf8_lossy(&input));
                    let parse = parse_uri(f);
                    let res = reqwest::blocking::get(&parse.uri).unwrap(); 
                    let body = res.text().unwrap(); 
                    stream.write(&body.as_bytes()).unwrap();
                    {
                        let mut of_file = of_file.lock().unwrap();
                        of_file.write_all(format!("{} {} {} {}\n",  now.to_rfc2822(), stream.local_addr().unwrap(), parse.uri, &body.as_bytes().len()).as_bytes()).ok();
                    }
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                break
            }
        }
    }
    // close the socket server
    drop(listener);
}
// GET http://www.bryancdixon.com:80/research/ HTTP/1.1
// GET http://www.aol.com:80/research/ HTTP/1.1
// GET http://www.nfl.com:80/ HTTP/1.1