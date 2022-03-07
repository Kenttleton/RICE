extern crate num_cpus;
use std::env;

pub struct Configuration {
    pub port: String,
    pub threads: usize,
    pub buffer_size: usize,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            port: get_port(),
            threads: get_threads(),
            buffer_size: get_buffer_size(),
        }
    }
}

fn get_port() -> String {
    let mut port = get_environment_variable("RICE_PORT");
    if !port.0 {
        port.1 = String::from("5000");
    }
    println!("Port: {}", port.1);
    port.1
}

fn get_threads() -> usize {
    let mut threads = get_environment_variable("RICE_THREADS");
    let max_threads = num_cpus::get() * 2;
    if !threads.0 || threads.1.parse::<usize>().unwrap() >= max_threads {
        threads.1 = (max_threads as f32 / 1.6).round().to_string();
    }
    println!("Threads: {}", threads.1.parse::<usize>().unwrap());
    threads.1.parse::<usize>().unwrap()
}

fn get_buffer_size() -> usize {
    let mut buffer_size = get_environment_variable("RICE_BUFFER_SIZE");
    if !buffer_size.0 {
        // 1 kb message size default
        buffer_size.1 = String::from("1024");
    }
    println!("Buffer Size: {} Bytes", buffer_size.1);
    buffer_size.1.parse::<usize>().unwrap()
}

fn get_environment_variable(match_key: &str) -> (bool, String) {
    let mut found = false;
    let mut value: String = String::default();
    for (_key, _value) in env::vars() {
        if _key == match_key {
            found = true;
            value = _value;
        }
    }
    (found, value)
}
