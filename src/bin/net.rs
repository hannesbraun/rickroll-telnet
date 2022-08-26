use rickroll_telnet::do_not_give_me_up;
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:23").expect("Unable to create TCP socket");
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    for stream in listener.incoming().flatten() {
        thread::spawn(|| do_not_give_me_up(Box::new(stream)));
    }
}
