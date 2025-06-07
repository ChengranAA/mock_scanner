use clap::{Arg, Command};
use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};
use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::{Duration, SystemTime},
};

fn handle_connection(stream: TcpStream) -> std::io::Result<bool> {
    let buf_reader = BufReader::new(stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .collect::<std::io::Result<Vec<_>>>()?;

    Ok(http_request.iter().any(|line| line == "Start"))
}

fn start_scanner(address: &str) -> std::io::Result<bool> {
    let listener = TcpListener::bind(address)?;
    for stream in listener.incoming() {
        let stream = stream?;
        if handle_connection(stream)? {
            return Ok(true);
        } else {
            return Ok(false)
        }
    }
    Ok(false)
}

fn main() {
    let matches = Command::new("mock_scanner")
        .about("A mock scanner utility")
        .version("1.0.0")
        .arg(
            Arg::new("tr")
                .long("tr")
                .short('t')
                .value_name("TR_VALUE")
                .help("Sets the TR value")
                .required(true)
                .value_parser(clap::value_parser!(f32)),
        )
        .arg(
            Arg::new("volumes")
                .long("volumes")
                .short('v')
                .value_name("VOLUMES")
                .help("Sets the volumes value")
                .required(true)
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .value_name("PORT")
                .help("Sets the port number (optional)")
                .required(false)
                .default_value("2333")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            Arg::new("trigger")
                .long("trigger")
                .value_name("TRIGGER")
                .help("Sets the trigger key (option)")
                .required(false)
                .default_value("5")
                .value_parser(clap::value_parser!(char)),
        )
        .get_matches();

    let address = format!("127.0.0.1:{}", matches.get_one::<u32>("port").unwrap());
    let tr = *matches.get_one::<f32>("tr").unwrap() as f64;
    let volumes = *matches.get_one::<u32>("volumes").unwrap() as u16;
    let trigger = *matches.get_one::<char>("trigger").unwrap();

    println!("Your current address: {}", address);

    match start_scanner(&address) {
        Ok(true) => {
            println!("Scanner Starting ...");
            thread::sleep(Duration::from_secs(1));

            let mut enigo = match Enigo::new(&Settings::default()) {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("Failed to initialize Enigo: {}", e);
                    return;
                }
            };

            let now = SystemTime::now();
            for _ in 0..volumes {
                if let Err(e) = enigo.key(Key::Unicode(trigger), Click) {
                    eprintln!("Failed to send key press: {}", e);
                    break;
                }
                thread::sleep(Duration::from_secs_f64(tr));
            }

            match now.elapsed() {
                Ok(elapsed) => {
                    println!("Finish Scanning ...");
                    let elapsed = elapsed.as_secs_f64();
                    let minutes = (elapsed / 60.0).floor();
                    let seconds = elapsed % 60.0;
                    println!("Elapsed time: {}m {:.3}s", minutes, seconds);
                }
                Err(e) => {
                    eprintln!("Error measuring elapsed time: {}", e);
                }
            }
        }
        Ok(false) => {
            eprintln!("No valid start command received");
        }
        Err(e) => {
            eprintln!("Failed to start scanner: {}", e);
        }
    }
}
