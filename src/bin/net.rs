use rickroll_telnet::{record, replay};
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

fn main() {
    let recording = Arc::new(record());
    let listener = TcpListener::bind("0.0.0.0:23").expect("Unable to create TCP socket");
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    for stream in listener.incoming().flatten() {
        let recording = Arc::clone(&recording);
        thread::spawn(move || {
            let _ = replay(Box::new(stream), &recording);
        });
    }
}
