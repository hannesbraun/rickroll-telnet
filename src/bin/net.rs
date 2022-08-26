use rickroll_telnet::{record, replay};
use std::net::TcpListener;
use std::sync::Arc;
use std::{env, thread};

fn main() {
    let port: u16 = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .map_or(23, |p| p.parse().unwrap_or(23));

    let recording = Arc::new(record());
    let listener =
        TcpListener::bind(format!("0.0.0.0:{}", port)).expect("Unable to create TCP socket");

    // Say hi
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    for stream in listener.incoming().flatten() {
        let recording = Arc::clone(&recording);
        thread::spawn(move || {
            let _ = replay(Box::new(stream), &recording);
        });
    }
}
