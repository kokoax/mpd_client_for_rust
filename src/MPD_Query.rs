pub mod MPD_Query {
    use std;
    use std::io::{Read, Write};

    // get tcp connection(socket) to mpd
    pub fn get_mpd_socket(addr: std::net::Ipv4Addr, port: u16) -> std::net::TcpStream {
        let mut mpd: std::net::TcpStream = std::net::TcpStream::connect(std::net::SocketAddrV4::new(addr, port))
            .expect("Failed get TCP socket(MPD_Query::get_mpd_socket)");

        // TODO without timeout
        // So, specify end keyword such.
        let _ = mpd.set_read_timeout(Some(std::time::Duration::new(0,1)));

        // Receive message("OK MPD $mpd_version") from mpd when after connect mpd, that throw to dustbox.
        let mut buf: std::string::String = std::string::String::new();
        let _ = mpd.read_to_string(&mut buf);

        return mpd;
    }

    // example. convert of "song: aaa\npath: /bbb/ccc\n" to {"song": "aaa", "path": "/bbb/ccc"}
    fn to_map(data: String) -> std::collections::HashMap<String, String> {
        let split_tmp: std::vec::Vec<&str> = data.split("\n").collect();
        let mut split_list: std::vec::Vec<Vec<&str>> = split_tmp
            .into_iter()
            .map(|x| x.split(": ").collect())
            .collect();

        // mpd send "OK" receive's last, that throw to dust.
        let _ = split_list.pop();
        let _ = split_list.pop();
        let mut map: std::collections::HashMap<String, String> = std::collections::HashMap::new();

        for i in 0..split_list.len() {
            map.insert(split_list[i][0].to_string(), split_list[i][1].to_string());
        }

        return map.clone();
    }

    // fn ls_to_map(data: String) -> std::vec::Vec<std::collections::HashMap<String, String>> {
    fn ls_to_map(data: String) -> std::collections::HashMap<String, String> {
        let split_tmp: std::vec::Vec<&str> = data.split("\n").collect();
        let mut split_list: std::vec::Vec<Vec<&str>> = split_tmp
            .into_iter()
            .map(|x| x.split(": ").collect())
            .collect();

        // mpd send "OK" receive's last, that throw to dust.
        let _ = split_list.pop();
        let _ = split_list.pop();
        let mut map: std::collections::HashMap<String, String> = std::collections::HashMap::new();

        for i in 0..split_list.len() {
            map.insert(split_list[i][0].to_string(), split_list[i][1].to_string());
        }

        return map.clone();
    }

    // get currentsong infomation
    pub fn currentsong(mpd: &mut std::net::TcpStream) -> std::collections::HashMap<String, String> {
        let mut buf: std::string::String = std::string::String::new();

        let _ = mpd.write(b"currentsong\n");
        let _ = mpd.read_to_string(&mut buf);

        return to_map(buf);
    }

    // fn ls(mpd: &mut TcpStream, path: &'static str) -> String {
    // fn ls(mpd: &mut TcpStream, path: &'static str) -> std::collections::HashMap<String, String> {
    // pub fn ls_dir(mpd: &mut std::net::TcpStream, path: &'static str) -> Vec<String> {
    // }
    // pub fn ls_file(mpd: &mut std::net::TcpStream, path: &'static str) -> Vec<String> {
    // }
    // pub fn ls_playlist(mpd: &mut std::net::TcpStream, path: &'static str) -> Vec<String> {
    // }
    // pub fn ls(mpd: &mut std::net::TcpStream, path: &'static str) -> std::vec::Vec<String> {
    pub fn ls(mpd: &mut std::net::TcpStream, path: &'static str) -> std::collections::HashMap<String, String> {
        let mut buf: std::string::String = std::string::String::new();

        let _ = mpd.write(format!("{} {}\n", "lsinfo", path).as_bytes());
        let _ = mpd.read_to_string(&mut buf);

        // return to_map(buf);
        let ls: std::vec::Vec<&str> = buf.split("\n").collect();

        let ret = ls.into_iter().map(|x| {
                 let tmp: std::vec::Vec<&str> = x.split(": ").collect();
                 let ret: std::string::String = tmp[1].to_string();
                 println!("{}", ret);
                 return ret;
            }).collect();
        return ret
    }
}

