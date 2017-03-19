extern crate regex;
extern crate gtk;
extern crate gdk_pixbuf;

mod main_window;
mod mpd;

// use std::net::{TcpStream,SocketAddrV4,Ipv4Addr};
use std::net::Ipv4Addr;
use gtk::prelude::*;

fn main() {
    let mut mpd = mpd::MPDQuery::new(Ipv4Addr::new(127,0,0,1), 6600);
    main_window::view(&mpd);
    // window::view();
}

