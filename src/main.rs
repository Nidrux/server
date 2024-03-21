
use std::io::{Read, Write};
use std::net::TcpListener;
use std::{env, process};

fn main() {
    /*Get the arguments passed along from the enviroment. */
    let args: Vec<String> = env::args().collect();
    if args.len() <= 0 {
        process::exit(1);
    }
    if args.len() > 1 {
        process::exit(1);
    }
    /*Defining the host and port */
    const HOST : &str = "127.0.0.1";
    const PORT : &str = "8080";
    /* Concat both to get the endpoint to_owned() -> Creates owned data from borrowed data, usually by cloning */
    let endpoint : String = HOST.to_owned() + ":" + PORT;
    /* Create the listener */
    let server = TcpListener::bind(endpoint.clone()).unwrap();
    println!("server is listening at port {}", PORT);
    println!("{}", endpoint);

    for stream in server.incoming() {
        let mut tcp_stream = stream.unwrap();
        println!("Connection established");
        let mut buffer = [0;1024];
        tcp_stream.read(&mut buffer).unwrap();
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        // const OK_RES : &str = "HTTP/1.1 200 OK\r\n\r\ntest"; /* CRLF https://developer.mozilla.org/en-US/docs/Glossary/CRLF */
        const NOTFOUND_RES : &str = "HTTP/1.1 404 ERROR\r\n\r\n{error: '404', message: 'not found'}";
        tcp_stream.write(NOTFOUND_RES.as_bytes()).unwrap();
        tcp_stream.flush().unwrap()
    }


}
