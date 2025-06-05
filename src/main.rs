use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};

use std::io::{BufReader, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::thread::sleep;
use std::{thread, time::Duration};

fn handle_connection(stream: TcpStream) -> bool {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if http_request.iter().any(|s| s == "Start") {
        true
    } else {
        false
    }
}

fn start_scanner(address: &str) -> bool {
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        if handle_connection(stream) {
            break;
        };
    }
    return true;
}

fn main() {
    let address = "127.0.0.1:7878";
    let tr: u64 = 2;
    let volumes: u16 = 5;
    let trigger = '5';

    println!("Your current address: {}", address);
    if start_scanner(address) {
        println!("Scanner Starting ...");
        sleep(Duration::from_secs(1));
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        let mut counter = 0;
        loop {
            if counter >= volumes {
                break;
            };
            enigo.key(Key::Unicode(trigger), Click).unwrap();
            thread::sleep(Duration::from_secs(tr));
            counter += 1;
        }
        println!("Finish Scanning ...")
    }
}
