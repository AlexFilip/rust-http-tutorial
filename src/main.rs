use std::{
    // str,
    convert::identity, io::{prelude::*, BufReader}, net::{SocketAddr, TcpListener, TcpStream}
};

use chrono::{/* NaiveDateTime, TimeZone, */ Utc};

fn main() {
    // let host = [127, 0, 0, 1];
    let host = [0, 0, 0, 0];
    let port = 7878;

    let host_str = host.map(|i| i.to_string()).join(".");
    let address_str = std::fmt::format(format_args!("{}:{}", host_str, port));
    println!("Listening on {}", address_str);

    let address = SocketAddr::from((host, port));

    match TcpListener::bind(address) {
        Ok(listener) =>
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        println!("Connection established");
                        // TODO: Handle error, if any
                        let _ = handle_connection(stream);

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

#[derive(Copy, Clone)]
struct ResponseCode {
    code: u16,
    message: &'static str
}

const CODE_200: ResponseCode = ResponseCode { code: 200, message: "OK" };
const CODE_400: ResponseCode = ResponseCode { code: 400, message: "Bad Request" };
// const CODE_403: ResponseCode = ResponseCode { code: 403, message: "Forbidden" };
const CODE_404: ResponseCode = ResponseCode { code: 404, message: "Not Found" };

#[derive(Copy, Clone)]
struct HTTPVersion {
    major: u8,
    minor: u8
}

const HTTP_VERSION_1_1: HTTPVersion = HTTPVersion { major: 1, minor: 1 };

// fn get_num() -> (u8, i32) {
//     (0, 0)
// }

fn parse_http_version(http_version_str: &str) -> Result<HTTPVersion, ()> {
    let start_of_version =  "HTTP/";
    if str::starts_with(http_version_str, start_of_version) {
        // let version_str = &http_version_str[start_of_version.len()..];
        // TODO: parse version string and return it as an HTTPVersion
        Ok(HTTP_VERSION_1_1)
    } else {
        Err(())
    }
}

fn create_response_string(http_version: HTTPVersion, response_code: ResponseCode, response_text: &str) -> String {
    let current_date_time = Utc::now();
    let response_code_str = format!("HTTP/{}.{} {} {}", http_version.major, http_version.minor, response_code.code, response_code.message);
    let response_date_str = format!("Date: {}", current_date_time.format("%a, %d %b %Y %H:%M:%S GMT"));
    let content_length_str = format!("Content-Length: {}", response_text.len());

    let response_arr_ = [
        response_code_str.as_str(),
        "Server: Custom",
        response_date_str.as_str(),
        content_length_str.as_str(),
        "Content-Type: text/html",
        "Cache-Control: no-store",
        "",
        response_text,
    ];

    let response_arr = if response_text.len() != 0 {
        &response_arr_
    } else {
        &response_arr_[..response_arr_.len() - 1]
    };

    println!("Response: {response_arr:#?}");

    let response = response_arr.join("\r\n");
    response
}

fn send_response(mut stream: TcpStream, response: String) {
    match stream.write(response.as_bytes()) {
        Ok(num_bytes) => println!("Wrote {} bytes", num_bytes),
        Err(err) => println!("Error: {}", err)
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), ()> {
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

    // TODO: Check if empty first
    let first_line_parts: Vec<_> = http_request[0].split(" ").collect();
    if first_line_parts.len() != 3 {
        let response = create_response_string(HTTP_VERSION_1_1, CODE_400, "First line does not have 3 parts");
        send_response(stream, response);
        return Ok(())
    }

    let _method = first_line_parts[0];
    let path = first_line_parts[1];
    let http_version_str = first_line_parts[2]; // Assume it's ok for now
    let http_version = parse_http_version(http_version_str)?;

    let response = if path == "/" {
        create_response_string(http_version, CODE_200, "X")
    } else {
        create_response_string(http_version, CODE_404, "")
    };

    send_response(stream, response);
    Ok(())
}

