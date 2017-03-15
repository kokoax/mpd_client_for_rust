extern crate regex;
extern crate gtk;

mod mpd;

// use std::net::{TcpStream,SocketAddrV4,Ipv4Addr};
use std::net::Ipv4Addr;
use mpd::mpd_query;
use gtk::prelude::*;

fn main() {
    // window_test();
    mpd_query_test();
}

fn window_test(){
    gtk::init()
        .expect("Failed to initialize GTK");

    let mut mpd = mpd_query::get_mpd_socket(Ipv4Addr::new(127,0,0,1), 6600);
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title("First");

    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350,70);

    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(false)
    });

    let column_types   = [gtk::Type::String, gtk::Type::String];
    let playlist_view  = gtk::TreeView::new();
    let playlist_store = gtk::ListStore::new(&column_types);

    let title_column_num = 0;
    let artist_column_num = 0;
    let title_column   = gtk::TreeViewColumn::new();
    title_column.set_title("Title");
    let artist_column   = gtk::TreeViewColumn::new();
    artist_column.set_title("Artist");

    playlist_view.append_column(&title_column);
    playlist_view.append_column(&artist_column);

    // playlist_store.set(&playlist_store.append(), &[0 as u32, 0 as u32], &[&"asd", &"dsa"]);
    // playlist_store.set(&playlist_store.insert(-1), &[0,1], &[&"asd", &"dsa"]);
    // let array_of_data = [&"asd".to_value() as &gtk::ToValue, &"dsa".to_value() as &gtk::ToValue];
    // let array_of_data = [&(("Title").to_value()) as &ToValue, &(("Artist").to_value()) as &ToValue];
    let iter = playlist_store.insert(-1);
    playlist_store.set_value(&iter, title_column_num, &"asd".to_value() as &gtk::Value);
    playlist_store.set_value(&iter, artist_column_num, &"dsa".to_value() as &gtk::Value);
    let iter = playlist_store.insert(-1);
    playlist_store.set_value(&iter, title_column_num, &"Sample".to_value() as &gtk::Value);
    playlist_store.set_value(&iter, artist_column_num, &"Elpmas".to_value() as &gtk::Value);

    // let iter = playlist_store.insert(-1);
    // playlist_store.set(&iter, &[0,1], &array_of_data);
    // let iter = playlist_store.insert_with_values(Some(0), &[0,1], &array_of_data);
    // let iter = playlist_store.insert_with_values(Some(0), &[0,1], &array_of_data);
    // playlist_store.set(&playlist_store.insert(-1), &[0,1], &[&"asd", &"dsa"]);
    // playlist_store.set_value(&iter, 1, &"asd".to_value() as &gtk::Value);
    playlist_view.set_model(Some(&playlist_store));


    window.add(&playlist_view);

    window.show_all();
    gtk::main();
}

fn mpd_query_test() {
    let mut mpd = mpd_query::get_mpd_socket(Ipv4Addr::new(127,0,0,1), 6600);

    // let ls_cmd = "\"ADAM at/CLOCK TOWER\"";
    let ls_cmd = "/";
    let ls              = mpd_query::ls(&mut mpd, ls_cmd);
    let ls_song         = mpd_query::ls_song(&mut mpd, ls_cmd);
    let ls_dir          = mpd_query::ls_dir(&mut mpd, ls_cmd);
    let ls_dir_and_song = mpd_query::ls_dir_and_song(&mut mpd, ls_cmd);
    let ls_playlist     = mpd_query::ls_playlist(&mut mpd, ls_cmd);
    let current         = mpd_query::currentsong(&mut mpd);
    let playlist        = mpd_query::playlist(&mut mpd);
    let playlistinfo    = mpd_query::playlistinfo(&mut mpd, "");
    let list            = mpd_query::list(&mut mpd, "");

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

    for item in playlist {
        for key in item.keys() {
            println!("{}: {}", key, item.get(key).unwrap());
        }
    }

    for item in playlistinfo {
        println!("Title: {}", item.get("Title").unwrap());
    }

    for item in list {
        println!("Album: {}", item);
    }
}

