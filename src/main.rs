extern crate regex;
extern crate gtk;

mod mpd;

// use std::net::{TcpStream,SocketAddrV4,Ipv4Addr};
use std::net::Ipv4Addr;
use mpd::mpd_query;

fn main() {
    test();
}

fn test() {
    let mut mpd = mpd_query::get_mpd_socket(Ipv4Addr::new(127,0,0,1), 6600);

    // let ls_cmd = "\"ADAM at/CLOCK TOWER\"";
    let ls_cmd = "/";
    let ls              = mpd_query::ls(&mut mpd, ls_cmd);
    let ls_song         = mpd_query::ls_song(&mut mpd, ls_cmd);
    let ls_dir          = mpd_query::ls_dir(&mut mpd, ls_cmd);
    let ls_dir_and_song = mpd_query::ls_dir_and_song(&mut mpd, ls_cmd);
    let ls_playlist     = mpd_query::ls_playlist(&mut mpd, ls_cmd);
    let current         = mpd_query::currentsong(&mut mpd);

    for item in ls_song {
        for key in item.keys() {
            println!("{}: {}", key, item.get(key).unwrap());
        }
        println!();
    }

    for item in ls_dir {
        for key in item.keys() {
            println!("{}: {}", key, item.get(key).unwrap());
        }
        println!();
    }

    for item in ls_playlist {
        for key in item.keys() {
            println!("{}: {}", key, item.get(key).unwrap());
        }
        println!();
    }

    for item in ls_dir_and_song {
        for key in item.keys() {
            println!("{}: {}", key, item.get(key).unwrap());
        }
        println!();
    }

    for item in ls {
        for key in item.keys() {
            println!("{}: {}", key, item.get(key).unwrap());
        }
        println!();
    }

    for key in current.keys() {
        println!("{}: {}", key, current.get(key).unwrap());
    }
}

