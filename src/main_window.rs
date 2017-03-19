extern crate gtk;
use gtk::prelude::*;

use std;
use mpd;
use gdk_pixbuf;

pub fn view(mpd: &mpd::MPDQuery) {
    gtk::init()
        .expect("Failed to initialize GTK");

    let mut main_window = gtk::Window::new(gtk::WindowType::Toplevel);
    init(&mut main_window);
    main_window.connect_delete_event(|_,_| {
        gtk::main_quit();
        gtk::prelude::Inhibit(false)
     });

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

fn get_new_column(title: &str, column_num: u32) -> gtk::TreeViewColumn {
    let column   = gtk::TreeViewColumn::new();
    column.set_title(title);

    let cell = gtk::CellRendererText::new();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", column_num as i32);

    return column;
}

fn get_playlist_window(mpd: &mpd::MPDQuery) -> gtk::ScrolledWindow {
    let playlistinfo    = mpd.playlistinfo("");

    let column_types   = [gtk::Type::String, gtk::Type::String];
    let playlist_view  = gtk::TreeView::new();
    let playlist_store = gtk::ListStore::new(&column_types);

    let title_column_num  = 0;
    let artist_column_num = 1;

    let title_column  = get_new_column("Title", title_column_num);
    let artist_column = get_new_column("Artist", artist_column_num);

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
        let pixbuf = match gdk_pixbuf::Pixbuf::new_from_file_at_size(&filepath, 150, 150) {
            Ok(pixbuf) => pixbuf,
            _          => gdk_pixbuf::Pixbuf::new_from_file_at_size(&noimage_path, 150, 150).unwrap(),
        };
        let image = gtk::Image::new_from_pixbuf(Some(&pixbuf));
        container.add(&image);
    }
}

fn get_album_window(mpd: &mpd::MPDQuery) -> gtk::ScrolledWindow {
    let flow = gtk::FlowBox::new();
    flow.set_valign(gtk::Align::Start);
    flow.set_column_spacing(20);
    flow.set_row_spacing(20);

    set_all_cover(mpd, &flow);

    let scroll = gtk::ScrolledWindow::new(None, None);
    scroll.add(&flow);

    return scroll;
}

fn get_inside_of_dir(path: &String, outside_iter: &gtk::TreeIter, store: &gtk::TreeStore, mpd: &mpd::MPDQuery) {
    let lsinfo = mpd.ls(&path);
    for ls in lsinfo {

        if ls.contains_key("directory") {
            let full_dirname = ls.get("directory").unwrap();
            let dirname = to_only_filename(full_dirname);

            let iter = store.insert_with_values(Some(&outside_iter), None, &[0], &[&dirname]);

            get_inside_of_dir(full_dirname, &iter, &store, mpd);
        } else if ls.contains_key("file") {
            let title = match ls.get("Title") {
                None        => to_only_filename(ls.get("file").unwrap()).to_value() as gtk::Value,
                Some(title) => title.to_value() as gtk::Value,
            };
            let artist = match ls.get("Artist") {
                None         => "".to_value() as gtk::Value,
                Some(artist) => artist.to_value() as gtk::Value,
            };
            let album = match ls.get("Album") {
                None         => "".to_value() as gtk::Value,
                Some(album) => album.to_value() as gtk::Value,
            };

            let _ = store.insert_with_values(Some(&outside_iter), None, &[0,1,2], &[&title,&artist,&album]);
        }
    }
}

fn get_music_dir_window(mpd: &mpd::MPDQuery) -> gtk::ScrolledWindow {
    let lsinfo = mpd.ls("");

    let column_types   = [gtk::Type::String, gtk::Type::String, gtk::Type::String];
    let dir_view  = gtk::TreeView::new();
    let dir_store = gtk::TreeStore::new(&column_types);

    let title_column_num  = 0;
    let artist_column_num = 1;
    let album_column_num = 2;

    let title_column  = get_new_column("Title", title_column_num);
    let artist_column = get_new_column("Artist", artist_column_num);
    let album_column = get_new_column("Album", album_column_num);

    dir_view.append_column(&title_column);
    dir_view.append_column(&artist_column);
    dir_view.append_column(&album_column);

    for ls in lsinfo {
        if ls.contains_key("directory") {
            let full_dirname = ls.get("directory").unwrap();
            let dirname = to_only_filename(full_dirname);

            let iter = dir_store.insert_with_values(None, None, &[0], &[&dirname]);

            get_inside_of_dir(full_dirname, &iter, &dir_store, mpd);
        } else if ls.contains_key("file") {
            let title = match ls.get("Title") {
                None        => to_only_filename(ls.get("file").unwrap()).to_value() as gtk::Value,
                Some(title) => title.to_value() as gtk::Value,
            };
            let artist = match ls.get("Artist") {
                None         => "".to_value() as gtk::Value,
                Some(artist) => artist.to_value() as gtk::Value,
            };
            let album = match ls.get("Album") {
                None         => "".to_value() as gtk::Value,
                Some(album) => album.to_value() as gtk::Value,
            };

            let _ = dir_store.insert_with_values(None, None, &[0,1,2], &[&title,&artist,&album]);
        }
    }

    dir_view.set_model(Some(&dir_store));

    let scroll = gtk::ScrolledWindow::new(None, None);
    scroll.add(&dir_view);

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

    let aw = get_album_window(mpd);
    stack.add_titled(&aw, "album", "Album");

    let mw = get_music_dir_window(mpd);
    stack.add_titled(&mw, "music_dir", "MusicDirectory");

    primary_box.pack_start(&stack_sidebar, false, true, 5);
    primary_box.pack_start(&stack, true, true, 5);

    return primary_box;
}

