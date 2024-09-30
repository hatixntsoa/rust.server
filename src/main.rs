use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};
use std::fs::File;
use std::thread;
use std::time::Duration;
use get_if_addrs::get_if_addrs;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    
    // Call the display_ip function to print the IP address as a clickable link
    display_ip();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn display_ip() {
    // Retrieve the current IPv4 address of the machine
    let local_ip = get_if_addrs()
        .unwrap()
        .iter()
        .filter_map(|iface| {
            // Check if the interface has an IPv4 address
            if iface.is_loopback() || iface.ip().is_ipv6() {
                None
            } else {
                Some(iface.ip().to_string())
            }
        })
        .next() // Get the first valid IPv4 address
        .unwrap_or_else(|| "127.0.0.1".to_string());

    // Print the IP address as a clickable link
    println!("Server running on : http://{}:7878", local_ip);
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "assets/rust.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "assets/rust.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "assets/not_found.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}