extern crate regex;
extern crate gtk;
extern crate gdk;
extern crate gdk_pixbuf;

mod main_window;
mod mpd;

// use std::net::{TcpStream,SocketAddrV4,Ipv4Addr};
use std::net::Ipv4Addr;

fn main() {
    let mpd = mpd::MPDQuery::new(Ipv4Addr::new(127,0,0,1), 6600);
    let main_window = main_window::MainWindow::new(mpd);
    main_window.view();
    // window::view();
}

