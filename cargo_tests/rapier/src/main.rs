use std::io::Write;
use std::io::{self, Read};
use std::net::TcpStream;

fn main() {
    // Specify the URL of the webpage you want to download
  


    // Establish a TCP connection to the server
    let mut stream = TcpStream::connect(("example.com:80")).unwrap();

    // Send an HTTP GET request
    let request = ("GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes()).unwrap();

    // Read the response
    let mut response = Vec::new();
    stream.read(&mut response).unwrap();

    // Print the content of the webpage
    std::io::stdout().write(&response);

   
}