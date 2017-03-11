mod MPD_Query;
// use std::net::{TcpStream,SocketAddrV4,Ipv4Addr};

fn main() {
    let mut mpd = MPD_Query::MPD_Query::get_mpd_socket(std::net::Ipv4Addr::new(127,0,0,1), 6600);

    let ls = MPD_Query::MPD_Query::ls(&mut mpd, "/");
    let current = MPD_Query::MPD_Query::currentsong(&mut mpd);

    println!("{}", ls.len());
    for i in 0..ls.len() {
        println!("{}", ls[i]);
    }

    for key in current.keys() {
        println!("{}: {}", key, current.get(key).unwrap());
    }
}
