use std::{
    error::Error,
    fs,
    io::{prelude::*, BufReader, Read},
    net::{TcpListener, TcpStream},
};

const HTTP_SERVER_PORT: &str = "7878";

fn main() {
    // Basic select to databse to test connectivity;
    match check_connectivity() {
        Ok(rows) => println!("DB connection check succeeded. Rows selected: {}", rows),
        Err(e) => {
            println!("DB connection check failed. Error: {}", e);
            std::process::exit(1);
        }
    }

    // Create listener
    let listener: TcpListener;
    match create_listener() {
        Ok(l) => listener = l,
        Err(e) => {
            println!("Failed to create TCP listener. Error: {}", e);
            std::process::exit(1);
        }
    }

    // Loop listening to incoming
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("Connection failed: Error: {}", e);
            }
        }
    }

    // Cleanup
    println!("Exiting.");
}

fn check_connectivity() -> Result<usize, Box<dyn Error>> {
    Ok(2)
}

fn create_listener() -> Result<TcpListener, std::io::Error> {
    let address = format!("127.0.0.1:{port}", port = HTTP_SERVER_PORT);

    TcpListener::bind(address)
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
