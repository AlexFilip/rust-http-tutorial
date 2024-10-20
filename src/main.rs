use std::{
    str,
    net::TcpListener
};

fn main() {
    match TcpListener::bind("127.0.0.1:7878") {
        Ok(listener) =>
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let mut buf = [0; 1000];
                        let size = match stream.peek(&mut buf) {
                            Ok(size) => size,
                            Err(error) =>  panic!("ERROR reading bytes: {}", error)
                        };
                        println!("Read {} bytes", size);

                        match str::from_utf8(&buf) {
                            Ok(contents) => {
                                println!("Connection established:\n\nCONTENTS:\n--------\n{}", contents);
                            }
                            Err(error) => {
                                panic!("ERROR reading UTF8: {}", error);
                            }
                        }
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
