use std::{
    // str,
    io::{prelude::*, BufReader},
    // io::Write,
    net::{SocketAddr, TcpListener, TcpStream}
};

fn main() {
    // let host = "127.0.0.1";
    // let host = "0.0.0.0";
    // let address = std::fmt::format(format_args!("{}:{}", host, port));

    let host = [0, 0, 0, 0];
    let port = 7878;
    let address = SocketAddr::from((host, port));

    match TcpListener::bind(address) {
        Ok(listener) =>
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        println!("Connection established");
                        handle_connection(stream);

                        // let mut buf = [0; 1000];
                        // let size = match stream.peek(&mut buf) {
                        //     Ok(size) => size,
                        //     Err(error) =>  panic!("ERROR reading bytes: {}", error)
                        // };
                        // println!("Read {} bytes", size);

                        // match str::from_utf8(&buf) {
                        //     Ok(contents) => {
                        //         // SUCCESSFUL CONNECTION
                        //         println!("Connection established:\n\nCONTENTS:\n--------\n{}", contents);
                        //         let bytes = "Hello world".as_bytes();
                        //         let _ = stream.write(&bytes);
                        //     }
                        //     Err(error) => {
                        //         panic!("ERROR reading UTF8: {}", error);
                        //     }
                        // }
                    }
                    Err(error) => {
                        println!("ERROR getting stream: {}", error);
                    }
                }
            }

        Err(error) => {
            println!("ERROR establishing connection: {}", error);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map_while(Result::ok)
        // .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}

