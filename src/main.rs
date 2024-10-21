use std::{
    // str,
    io::{prelude::*, BufReader},
    // io::Write,
    net::{TcpListener, TcpStream}
};

fn main() {
    match TcpListener::bind("127.0.0.1:7878") {
        Ok(listener) =>
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
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

