use std::string::String;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut buf = String::new();

    let mut mpd = get_mpd_socket(6600);
    let _ = mpd.write(b"currentsong\n");
    let _ = mpd.read_to_string(&mut buf);

    println!("{}", buf);
}

fn get_mpd_socket(port: u32) {
    let mut mpd = TcpStream::connect(format!("{}{}", "localhost:", port.to_string()).unwrap();
    // TODO without timeout
    // So, specify end keyword such.
    mpd.set_read_timeout(Some(std::time::Duration::new(0,1)))
        .expect("faild set read timeoutl");

    return mpd;
}

fn ls(path: String) {
    println!("{}", path);
}

