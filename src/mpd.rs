pub mod mpd_query {
    use std;
    use std::io::{Read, Write};
    use std::vec::Vec;
    use std::string::String;
    use std::collections::HashMap;
    use std::net::{TcpStream, Ipv4Addr,SocketAddrV4};
    extern crate regex;

    // get tcp connection(socket) to mpd
    pub fn get_mpd_socket(addr: Ipv4Addr, port: u16) -> TcpStream {
        let mut mpd: TcpStream = TcpStream::connect(SocketAddrV4::new(addr, port))
            .expect("Failed get TCP socket(MPD_Query::get_mpd_socket)");

        // TODO without timeout
        // So, specify end keyword such.
        let _ = mpd.set_read_timeout(Some(std::time::Duration::new(0,1)));

        // Receive message("OK MPD $mpd_version") from mpd when after connect mpd, that throw to dustbox.
        let mut buf: String = String::new();
        let _ = mpd.read_to_string(&mut buf);

        return mpd;
    }

    // MPD receive data(String) to vector<hashmap>
    fn mpdbuf_to_vec(buf: String) -> Vec<HashMap<String, String>> {
        let mut ret: Vec<HashMap<String, String>> = Vec::<HashMap<String, String>>::new();
        let mut ls: Vec<&str> = buf.split("\n").collect();
        ls.pop();  // "OK\n".split("\n") -> ["OK", ""].pop()

        let mut ls_data: HashMap<String, String> = HashMap::<String, String>::new();

        // The buffer's last line is "OK"
        let is_last = regex::Regex::new(r"OK").unwrap();
        // The buffer's top line is "file" or "directory" or "playlist"
        let is_top_attr = regex::Regex::new(r"file|directory|playlist").unwrap();
        for line in ls {
            if !is_last.is_match(line) {
                // ex:item. "file: ~/Music/Sample.mp3".splite(": ") -> ["file", "~/Music/Sample.mp3"]
                let splited: Vec<&str> = line.split(": ").collect();
                if is_top_attr.is_match(splited[0]) {
                    ret.push(ls_data.clone());
                    ls_data.clear();
                }
                ls_data.insert(splited[0].to_string(), splited[1].to_string());
            }else{
                ret.push(ls_data.clone());
            }
        }
        ret.remove(0);
        return ret;
    }

    // get currentsong infomation
    pub fn currentsong(mpd: &mut TcpStream) -> HashMap<String, String> {
        let mut buf: String = String::new();

        let _ = mpd.write(b"currentsong\n");
        let _ = mpd.read_to_string(&mut buf);

        return mpdbuf_to_vec(buf).pop().unwrap();
    }

    // get list any types(song, album, artist, etc...)
    pub fn list(mpd: &mut TcpStream, filter: &str) -> Vec<String> {
        let mut buf: String = String::new();

        let _ = mpd.write(format!("{} {}\n", "list", filter).as_bytes());
        let _ = mpd.read_to_string(&mut buf);

        let splited: Vec<&str> = buf.split("\n").collect();

        let mut ret: Vec<String> = splited.into_iter().map(|x| {
            match x {
                "OK" => "OK".to_string(),
                "" => "NL".to_string(),
                _      => {
                    println!("{}", x);
                    let splited: Vec<&str> = x.split(": ").collect();
                    println!("after");
                    splited[1].to_string()
                },
            }
        }).collect();
        ret.remove(0);
        ret.pop();
        ret.pop();
        return ret;
    }

    pub fn playlistinfo(mpd: &mut TcpStream, songpos: &str) -> Vec<HashMap<String, String>> {
        let mut buf: String = String::new();

        let _ = mpd.write(format!("{} {}\n", "playlistinfo", songpos).as_bytes());
        let _ = mpd.read_to_string(&mut buf);

        return mpdbuf_to_vec(buf);
    }

    pub fn playlist(mpd: &mut TcpStream) -> Vec<HashMap<String, String>> {
        let mut buf: String = String::new();

        let _ = mpd.write(b"playlist\n");
        let _ = mpd.read_to_string(&mut buf);

        return mpdbuf_to_vec(buf);
    }

    // get only directory from ls
    pub fn ls_dir(mpd: &mut TcpStream, path: &'static str) -> Vec<HashMap<String, String>> {
        let mut ls_dir: Vec<HashMap<String, String>> = ls(mpd, path);
        ls_dir.retain(|item| item.contains_key("directory"));
        return ls_dir;
    }
    // get only directory from ls
    pub fn ls_song(mpd: &mut TcpStream, path: &'static str) -> Vec<HashMap<String, String>> {
        let mut ls_dir_and_song: Vec<HashMap<String, String>> = ls(mpd, path);
        ls_dir_and_song.retain(|item| item.contains_key("file"));
        return ls_dir_and_song;
    }
    // get only playlist from ls
    pub fn ls_playlist(mpd: &mut TcpStream, path: &'static str) -> Vec<HashMap<String, String>> {
        let mut ls_playlist: Vec<HashMap<String, String>> = ls(mpd, path);
        ls_playlist.retain(|item| item.contains_key("playlist"));
        return ls_playlist;
    }
    // get directory and song from ls
    pub fn ls_dir_and_song(mpd: &mut TcpStream, path: &'static str) -> Vec<HashMap<String, String>> {
        let mut ls_dir_and_song: Vec<HashMap<String, String>> = ls(mpd, path);
        ls_dir_and_song.retain(|item| item.contains_key("file") || item.contains_key("directory"));
        return ls_dir_and_song;
    }
    // get mpd' ls command result
    pub fn ls(mpd: &mut TcpStream, path: &'static str) -> Vec<HashMap<String, String>> {
        let mut buf: String = String::new();

        let _ = mpd.write(format!("{} {}\n", "lsinfo", path).as_bytes());
        let _ = mpd.read_to_string(&mut buf);

        return mpdbuf_to_vec(buf);
   }
}

