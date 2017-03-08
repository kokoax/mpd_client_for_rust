use std::string::String;
use std::io::prelude::*;
use std::net::{TcpStream,SocketAddrV4,Ipv4Addr};

fn main() {
    let mut mpd: TcpStream = get_mpd_socket(Ipv4Addr::new(127,0,0,1), 6600);
    to_map("asd: sss\n ddd: www".to_string());
}

fn get_mpd_socket(addr: Ipv4Addr, port: u16) -> TcpStream {
    // TODO without timeout
    // So, specify end keyword such.
    let mut mpd: TcpStream = TcpStream::connect(SocketAddrV4::new(addr, port)).unwrap();
    let _ = mpd.set_read_timeout(Some(std::time::Duration::new(0,1)));

    // Receive message("OK MPD $mpd_version") from mpd when after connect mpd. Throw to dustbox.
    let mut buf: String = String::new();
    let _ = mpd.read_to_string(&mut buf);

    return mpd;
}

fn to_map(data: String) -> std::collections::HashMap<String, String> {
    let split_tmp: Vec<&str> = data.split("\n").collect();
    let split_list: Vec<Vec<&str>> = split_tmp
                            .into_iter()
                            .map(|x| x.split(": ").collect())
                            .collect();

    let mut map: &std::collections::HashMap<String, String> = &std::collections::HashMap::new();

    // split_list.into_iter().map(|x| map.insert(x[0].to_string(), x[1].to_string()));
    split_list
        .into_iter()
        .map(|x| map.insert(x[0].to_string(), x[1].to_string()));
    // split_list.into_iter().map(|x| println!("asd: {}", x));

    // println!("split_list: {}", split_list.len());
    println!("map: {}", map.len());

    return map.clone();
}

fn currentsong(mpd: &mut TcpStream) -> String {
    let mut buf: String = String::new();

    let _ = mpd.write(b"currentsong\n");
    let _ = mpd.read_to_string(&mut buf);

    return buf;
}

fn ls(mpd: &mut TcpStream, path: &'static str) -> String {
    let mut buf: String = String::new();

    let _ = mpd.write(format!("{} {}\n", "lsinfo", path).as_bytes());
    let _ = mpd.read_to_string(&mut buf);

    return buf;
}

