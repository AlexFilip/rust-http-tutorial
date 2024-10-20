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
                        match stream.peek(&mut buf) {
                            Ok(size) => println!("Read {} bytes", size),
                            Err(error) =>  println!("ERROR reading bytes: {}", error)
                        };

                        let contents = match str::from_utf8(&buf) {
                            Ok(s) => s,
                            Err(error) => {
                                println!("ERROR reading UTF8: {}", error);
                                ""
                            }
                        };
                        println!("Connection established:\n\nCONTENTS:\n--------\n{}", contents);
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
