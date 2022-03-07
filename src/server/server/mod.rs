extern crate ctrlc;
use crate::configuration::Configuration;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
mod threading;
use threading::ThreadPool;

pub struct Server {
    listener: Option<TcpListener>,
    terminate: bool,
    handle_connection: Option<fn(TcpStream)>,
    configuration: Option<Configuration>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            listener: None,
            terminate: false,
            handle_connection: None,
            configuration: None,
        }
    }

    pub fn bind(&mut self, config: Configuration) -> &mut Self {
        let address: String = format!("{}:{}", "127.0.0.1", config.port.clone());
        self.listener = Some(TcpListener::bind(address).unwrap());
        self.configuration = Some(config);
        self
    }

    pub fn run(&mut self, handler: fn(TcpStream)) {
        let pool = ThreadPool::new(16);
        for stream in self.listener.unwrap().incoming() {
            let stream = stream.unwrap();
            pool.spawn(move || {
                handler(stream);
            });
            if self.terminate {
                break;
            }
        }
    }

    pub fn kill(&mut self) {
        self.terminate = true;
        // TODO close all connections
    }
}

// fn stream_handler(mut stream: TcpStream) {
//     let mut buffer: Vec<u8> = Vec::new();
//     let res = stream.read(&mut buffer);
//     let (request, response) = match res {
//         Result::Err(_) => (Request::new(), Response::new()),
//         Result::Ok(_) => parse_request(&mut buffer),
//     };
//     let mut response = handle_request(request, response);
//     stream.write(response.to_string().as_bytes()).unwrap();
//     stream.flush().unwrap();
// }

// fn parse_request(buffer: &mut [u8]) -> (Request, Response) {
//     //println!("{}", String::from_utf8_lossy(&buffer[..]));
//     let iter = buffer.iter().copied();
//     let parser = Parser::new(iter);
//     (parser.parse(), Response::new())
// }

// fn handle_request(mut request: Request, mut response: Response) -> Response {
//     print!("{:#?}\n{:#?}\n", request, response);
//     response
// }
