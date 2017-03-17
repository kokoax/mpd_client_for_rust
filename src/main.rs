extern crate regex;
extern crate gtk;

mod mpd;

// use std::net::{TcpStream,SocketAddrV4,Ipv4Addr};
use std::net::Ipv4Addr;
use mpd::mpd_query;
use gtk::prelude::*;

fn main() {
    // window_test();
    menu_bar();
    // mpd_query_test();
}

fn to_only_filename(data: &String) -> String {
    let mut splited: Vec<&str> = data.split("/").collect();
    return splited.pop().unwrap().to_string() as String;
}

fn menu_bar() {
    gtk::init()
        .expect("Failed to initialize GTK");

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title("mpd_client");

    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350,70);

    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(false)
    });

    /* main Box */
    let primary_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    /* Stack */
    let stack = gtk::Stack::new();
    stack.set_transition_type(gtk::StackTransitionType::SlideLeft);

    /* StackSwitch */
    let stack_switcher = gtk::StackSwitcher::new();
    stack_switcher.set_stack(Some(&stack));

    /* StackSidebar */
    let stack_sidebar = gtk::StackSidebar::new();
    stack_sidebar.set_stack(&stack);

    /* Stack inner */
    let tv = gtk::TextView::new();
    let buf = tv.get_buffer().unwrap();
    buf.set_text("Text");
    stack.add_titled(&tv, "view", "View");
    stack.add(&tv);

    let label = gtk::Label::new("asd");

    stack.add_titled(&label, "label", "Label");
    stack.add(&label);

    primary_box.pack_start(&stack_sidebar, true, true, 5);
    primary_box.pack_start(&stack, true, true, 5);

    /* main */
    window.add(&primary_box);

    window.show_all();
    gtk::main();
}

fn window_test(){
    gtk::init()
        .expect("Failed to initialize GTK");

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title("mpd_client");

    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350,70);

    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(false)
    });

    let mut mpd = mpd_query::get_mpd_socket(Ipv4Addr::new(127,0,0,1), 6600);
    let playlistinfo    = mpd_query::playlistinfo(&mut mpd, "");

    let column_types   = [gtk::Type::String, gtk::Type::String];
    let playlist_view  = gtk::TreeView::new();
    let playlist_store = gtk::ListStore::new(&column_types);

    let title_column_num  = 0;
    let artist_column_num = 1;

    let title_column   = gtk::TreeViewColumn::new();
    title_column.set_title("Title");
    let title_cell = gtk::CellRendererText::new();
    title_column.pack_start(&title_cell, true);
    title_column.add_attribute(&title_cell, "text", title_column_num as i32);

    let artist_column   = gtk::TreeViewColumn::new();
    artist_column.set_title("Artist");
    let artist_cell = gtk::CellRendererText::new();
    artist_column.pack_start(&artist_cell, true);
    artist_column.add_attribute(&artist_cell, "text", artist_column_num as i32);

    playlist_view.append_column(&title_column);
    playlist_view.append_column(&artist_column);

    for info in playlistinfo {
        let iter = playlist_store.insert(-1);
        let title = match info.get("Title") {
            None        => to_only_filename(info.get("file").unwrap()).to_value() as gtk::Value,
            Some(title) => title.to_value() as gtk::Value,
        };
        let artist = match info.get("Artist") {
            None         => "".to_value() as gtk::Value,
            Some(artist) => artist.to_value() as gtk::Value,
        };
        playlist_store.set_value(&iter, title_column_num,  &title);
        playlist_store.set_value(&iter, artist_column_num, &artist);
    }

    playlist_view.set_model(Some(&playlist_store));

    let scroll = gtk::ScrolledWindow::new(None, None);
    scroll.add(&playlist_view);

    // window.add(&playlist_view);
    window.add(&scroll);

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
    let list            = mpd_query::list(&mut mpd, "album");

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
        println!("Title : {}", item.get("Title").unwrap());
        println!("Artist: {}", item.get("Artist").unwrap());
    }

    for item in list {
        println!("Album: {}", item);
    }
}

