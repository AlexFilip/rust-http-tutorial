use std::{
    // str,
    convert::identity, io::{prelude::*, BufReader}, net::{SocketAddr, TcpListener, TcpStream}
};

fn main() {
    // let host = "127.0.0.1";
    // let host = "0.0.0.0";
    // let address = std::fmt::format(format_args!("{}:{}", host, port));

    // let host = [127, 0, 0, 1];
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
        .flat_map(identity)
        // .map(|result| result.unwrap())
        // take_while actually terminates the connection, since it continues to wait until it
        // reaches a newline
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");

    let response = [
        "HTTP/1.1 200 OK",
        "Server: Custom",
        "Date: Wed, 12 Feb 2025 10:37:50 EST", // temporary
        "Content-Length: 1", // temporary
        "Content-Type: text/html",
        "Cache-Control: no-store",
        "",
        "X"
    ].join("\n");

    match stream.write(response.as_bytes()) {
        Ok(num_bytes) => println!("Wrote {} bytes", num_bytes),
        Err(err) => println!("Error: {}", err)
    }
}

