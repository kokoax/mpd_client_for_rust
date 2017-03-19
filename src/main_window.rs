extern crate gtk;
use gtk::prelude::*;

use std;
use mpd;
use mpd::MPDQuery;
use gdk_pixbuf;

use std::net::{TcpStream, Ipv4Addr,SocketAddrV4};

pub fn view(mpd: &mpd::MPDQuery) {
    gtk::init()
        .expect("Failed to initialize GTK");

    let mut main_window = gtk::Window::new(gtk::WindowType::Toplevel);
    init(&mut main_window);
    main_window.connect_delete_event(|_,_| {
        gtk::main_quit();
        gtk::prelude::Inhibit(false)
     });

    let label = gtk::Label::new("asd");

    main_window.add(&get_main_box(mpd));

    main_window.show_all();
    gtk::main();
}


fn to_only_filename(data: &String) -> String {
    let mut splited: Vec<&str> = data.split("/").collect();
    return splited.pop().unwrap().to_string() as String;
}

fn init(main_window: &gtk::Window) {
    main_window.set_title("mpd_client");

    main_window.set_border_width(10);
    main_window.set_position(gtk::WindowPosition::Center);
    main_window.set_default_size(350,70);
}

fn get_playlist_window(mpd: &mpd::MPDQuery) -> gtk::ScrolledWindow {
    let playlistinfo    = mpd.playlistinfo("");

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
        let title = match info.get("Title") {
            None        => to_only_filename(info.get("file").unwrap()).to_value() as gtk::Value,
            Some(title) => title.to_value() as gtk::Value,
        };
        let artist = match info.get("Artist") {
            None         => "".to_value() as gtk::Value,
            Some(artist) => artist.to_value() as gtk::Value,
        };
        playlist_store.insert_with_values(Some(0), &[title_column_num, artist_column_num], &[&title, &artist]);
    }

    playlist_view.set_model(Some(&playlist_store));

    let scroll = gtk::ScrolledWindow::new(None, None);
    scroll.add(&playlist_view);

    return scroll;
}

fn set_all_cover(mpd: &mpd::MPDQuery, container: &gtk::FlowBox) {
    // TODO May be different of behavior windows and linux
    let home = std::env::home_dir().unwrap().into_os_string().into_string().unwrap();

    let noimage_path = format!("{}/.cache/mpd_client/cover/noimage.png", home);
    let album_array = mpd.list("Album");
    let album_cover_paths: Vec<String> = album_array.into_iter()
        .map(|item| format!("{}/.cache/mpd_client/cover/{}.png", home, item)).collect();

    for filepath in album_cover_paths {
        println!("{}", filepath);
        let colorspace: gdk_pixbuf::Colorspace = 0;
        let pixbuf = match gdk_pixbuf::Pixbuf::new_from_file_at_size(&filepath, 150, 150) {
            Ok(pixbuf) => pixbuf,
            _          => gdk_pixbuf::Pixbuf::new_from_file_at_size(&noimage_path, 150, 150).unwrap(),
        };
        let image = gtk::Image::new_from_pixbuf(Some(&pixbuf));
        container.add(&image);
    }
}

fn get_album_window(mpd: &mpd::MPDQuery) -> gtk::ScrolledWindow {
    let mut flow = gtk::FlowBox::new();
    flow.set_valign(gtk::Align::Start);
    flow.set_column_spacing(20);
    flow.set_row_spacing(20);

    set_all_cover(mpd, &flow);

    let scroll = gtk::ScrolledWindow::new(None, None);
    scroll.add(&flow);

    return scroll;
}

fn get_main_box(mpd: &mpd::MPDQuery) -> gtk::Box {
    /* main Box */
    let primary_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    /* Stack */
    let stack = gtk::Stack::new();
    stack.set_transition_type(gtk::StackTransitionType::SlideLeft);

    /* StackSidebar */
    let stack_sidebar = gtk::StackSidebar::new();
    stack_sidebar.set_stack(&stack);

    /* Stack inner */
    let pw = get_playlist_window(mpd);
    stack.add_titled(&pw, "playlist", "Playlist");
    stack.add(&pw);

    let mut aw = get_album_window(mpd);
    stack.add_titled(&aw, "album", "Album");
    stack.add(&aw);

    primary_box.pack_start(&stack_sidebar, false, true, 5);
    primary_box.pack_start(&stack, true, true, 5);

    return primary_box;
}

fn mpd_query_test() {
    let mut mpd: mpd::MPDQuery = mpd::MPDQuery::new(Ipv4Addr::new(127,0,0,1), 6600);

    // let ls_cmd = "\"ADAM at/CLOCK TOWER\"";
    let ls_cmd = "/";
    let ls              = mpd.ls(ls_cmd);
    let ls_song         = mpd.ls_song(ls_cmd);
    let ls_dir          = mpd.ls_dir(ls_cmd);
    let ls_dir_and_song = mpd.ls_dir_and_song(ls_cmd);
    let ls_playlist     = mpd.ls_playlist(ls_cmd);
    let current         = mpd.currentsong();
    let playlist        = mpd.playlist();
    let playlistinfo    = mpd.playlistinfo("");
    let list            = mpd.list("album");

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

