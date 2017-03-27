extern crate gtk;
extern crate gdk;

use std::sync::Mutex;

use std;
use mpd;
use gdk_pixbuf;

use gtk::prelude::*;

pub struct MainWindow {
    mpd: Mutex<mpd::MPDQuery>
}


impl MainWindow {
    pub fn new(mpd_org: mpd::MPDQuery) -> MainWindow {
        let mutex_mpd_org = Mutex::<mpd::MPDQuery>::new(mpd_org);
        MainWindow{mpd: mutex_mpd_org}
    }

    fn init(&self, main_window: &gtk::Window) {
        main_window.set_title("mpd_client");

        main_window.set_border_width(10);
        main_window.set_position(gtk::WindowPosition::Center);
        main_window.set_default_size(350,70);
    }

    pub fn view(&self) {
        println!("init");
        gtk::init()
            .expect("Failed to initialize GTK");

        let mut main_window = gtk::Window::new(gtk::WindowType::Toplevel);
        let mut all_container = gtk::Box::new(gtk::Orientation::Vertical, 10);

        self.init(&mut main_window);


        main_window.connect_delete_event(|_,_| {
            gtk::main_quit();
            gtk::prelude::Inhibit(false)
        });

        println!("create window");
        all_container.pack_start(&self.get_header(),   false, true, 5);
        all_container.pack_start(&self.get_main_box(),  true, true, 5);

        main_window.add(&all_container);

        println!("show");
        main_window.show_all();
        gtk::main();
    }

    fn to_only_filename(data: &String) -> String {
        let mut splited: Vec<&str> = data.split("/").collect();
        return splited.pop().unwrap().to_string() as String;
    }


    fn get_new_column(&self, title: &str, column_num: u32) -> gtk::TreeViewColumn {
        let column   = gtk::TreeViewColumn::new();
        column.set_title(title);

        let cell = gtk::CellRendererText::new();

        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", column_num as i32);

        return column;
    }

    fn get_header(&self) -> gtk::Box {
        let header = gtk::Box::new(gtk::Orientation::Horizontal, 10);

        let seek_bar_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let seek_bar_adj = gtk::Adjustment::new(100.0, 0.0, 100.0, 1.0, 10.0, 0.0);
        let seek_bar = gtk::ProgressBar::new();
        // seek_bar.set_draw_value(false);
        seek_bar.set_fraction(0.3);
        // seek_bar.set_pulse_step(0.5);

        let controll_button_box = gtk::Box::new(gtk::Orientation::Horizontal, 2);
        controll_button_box.set_halign(gtk::Align::Center);

        let prev_button = gtk::Button::new_with_label("<<");
        let stop_button = gtk::Button::new_with_label("||");
        let play_button = gtk::Button::new_with_label("|>");
        let next_button = gtk::Button::new_with_label(">>");

        controll_button_box.pack_start(&prev_button,  false, false, 2);
        controll_button_box.pack_start(&stop_button,  false, false, 2);
        controll_button_box.pack_start(&play_button,  false, false, 2);
        controll_button_box.pack_start(&next_button,  false, false, 2);

        // let play_time   = gtk::Label::new();

        seek_bar_box.pack_start(&seek_bar, true, true, 0);

        header.pack_start(&seek_bar_box,  true, true, 2);
        header.pack_start(&controll_button_box, false, false, 2);

        let locked_mpd  = self.mpd.lock().unwrap();
        let mpd_timeout = locked_mpd.clone();
        gtk::timeout_add_seconds(1, move || {
            let mut mpd = mpd_timeout.clone();
            let seek_bar = seek_bar.clone();
            let time_buf = mpd.status();
            let time_buf: Vec<&str> = time_buf.get("time")
                .unwrap()
                .split(":")
                .collect();
            let time_buf: Vec<f64> = time_buf.into_iter()
                .map(|item| item.parse::<f64>().unwrap()).collect();

            let time = time_buf[0] / time_buf[1];
            seek_bar.set_fraction(time);

            gtk::Continue(true)
        });
        drop(locked_mpd);

        return header;
    }

    fn get_playlist_store(playlistinfo: Vec<std::collections::HashMap<String, String>>) -> gtk::ListStore {
        let column_types   = [gtk::Type::String, gtk::Type::String, gtk::Type::String];
        let playlist_store = gtk::ListStore::new(&column_types);

        for info in playlistinfo {
            let title = match info.get("Title") {
                None        => MainWindow::to_only_filename(info.get("file").unwrap()).to_value() as gtk::Value,
                Some(title) => title.to_value() as gtk::Value,
            };
            let artist = match info.get("Artist") {
                None         => "".to_value() as gtk::Value,
                Some(artist) => artist.to_value() as gtk::Value,
            };
            let album = match info.get("Album") {
                None         => "".to_value() as gtk::Value,
                Some(album) => album.to_value() as gtk::Value,
            };

            playlist_store.insert_with_values(
                Some(0),
                &[0,1,2],
                &[&title, &artist, &album]);
        }

        return playlist_store;
    }

    fn get_playlist_window(&self) -> gtk::ScrolledWindow {
        let mut mpd = self.mpd.lock().unwrap();
        let mut playlistinfo   = mpd.playlistinfo("");
        drop(mpd);
        playlistinfo.reverse();

        let playlist_view  = gtk::TreeView::new();
        let playlist_store = MainWindow::get_playlist_store(playlistinfo);

        let title_column_num  = 0;
        let artist_column_num = 1;
        let album_column_num  = 2;

        let title_column  = self.get_new_column("Title", title_column_num);
        let artist_column = self.get_new_column("Artist", artist_column_num);
        let album_column  = self.get_new_column("Album", album_column_num);

        playlist_view.append_column(&title_column);
        playlist_view.append_column(&artist_column);
        playlist_view.append_column(&album_column);

        playlist_view.set_model(Some(&playlist_store));
        playlist_view.set_enable_search(false);
        playlist_view.get_selection().set_mode(gtk::SelectionMode::Multiple);

        // TODO: more better than that way(mpd clone).
        let locked_mpd = self.mpd.lock().unwrap();
        let mut mpd_forcus   = locked_mpd.clone();
        let mut mpd_keypress = locked_mpd.clone();
        playlist_view.connect_focus_in_event(move |widget, _| {
            let mpd = mpd_forcus.clone();
            let mut playlistinfo = mpd.playlistinfo("");
            playlistinfo.reverse();
            let playlist_store = MainWindow::get_playlist_store(playlistinfo);
            widget.set_model(Some(&playlist_store));
            gtk::prelude::Inhibit(false)
        });
        playlist_view.connect_key_press_event(move |widget,event_key| {
            // TODO: more better than that way.
            let mpd = mpd_keypress.clone();
            match event_key.get_keyval() as u32 {
                // 65535 => {  // Delete key
                100 => {  // 100 means 'd'
                    let (paths, _) = widget.get_selection().get_selected_rows();
                    let mut gap = 0;  // gap of deleting song in playlist(To move forward under the song).
                    for path in paths {
                        for index in path.get_indices() {
                            mpd.delete(&(index-gap).to_string());
                            gap += 1;
                        }
                    }
                    let mut playlistinfo = mpd.playlistinfo("");
                    playlistinfo.reverse();
                    let playlist_store = MainWindow::get_playlist_store(playlistinfo);
                    widget.set_model(Some(&playlist_store));
                },
                _     => println!("Other keyval"),
            }
            gtk::prelude::Inhibit(false)
        });
        drop(locked_mpd);

        let scroll = gtk::ScrolledWindow::new(None, None);
        scroll.add(&playlist_view);

        return scroll;
    }

    fn set_all_cover(&self, container: &gtk::FlowBox) {
        let  mpd = self.mpd.lock().unwrap();
        let album_array = mpd.list("Album");
        drop(mpd);
        // TODO May be different of behavior windows and linux
        let home = std::env::home_dir().unwrap().into_os_string().into_string().unwrap();

        println!("album_len: {}", album_array.len());
        let noimage_path = format!("{}/.cache/mpd_client/cover/noimage.png", home);
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

    fn get_album_window(&self) -> gtk::ScrolledWindow {
        let flow = gtk::FlowBox::new();
        flow.set_valign(gtk::Align::Start);
        flow.set_column_spacing(20);
        flow.set_row_spacing(20);

        self.set_all_cover(&flow);

        flow.connect_child_activated(|flowbox, flowbox_child| {
            println!("children per line: {}", flowbox.get_column_spacing());
        });

        let label = gtk::Label::new("ASd");
        flow.insert(&label, 5);

        let scroll = gtk::ScrolledWindow::new(None, None);
        scroll.add(&flow);

        return scroll;
    }

    fn get_inside_of_dir(&self, path: &String, outside_iter: &gtk::TreeIter, store: &gtk::TreeStore) {
        let  mpd = self.mpd.lock().unwrap();
        let lsinfo = mpd.ls(&path);
        drop(mpd);
        for ls in lsinfo {
            if ls.contains_key("directory") {
                let full_dirname = ls.get("directory").unwrap();
                let dirname = MainWindow::to_only_filename(full_dirname);

                let iter = store.insert_with_values(Some(&outside_iter), None, &[0], &[&dirname]);

                self.get_inside_of_dir(full_dirname, &iter, &store);
            } else if ls.contains_key("file") {
                let filename = MainWindow::to_only_filename(ls.get("file").unwrap()).to_value() as gtk::Value;

                let artist = match ls.get("Artist") {
                    None         => "".to_value() as gtk::Value,
                    Some(artist) => artist.to_value() as gtk::Value,
                };
                let album = match ls.get("Album") {
                    None         => "".to_value() as gtk::Value,
                    Some(album) => album.to_value() as gtk::Value,
                };

                let _ = store.insert_with_values(Some(&outside_iter), None, &[0,1,2], &[&filename,&artist,&album]);
            }
        }
    }

    fn get_music_dir_window(&self) -> gtk::ScrolledWindow {
        let mut mpd = self.mpd.lock().unwrap();
        let lsinfo = mpd.ls("");
        drop(mpd);

        let column_types   = [gtk::Type::String, gtk::Type::String, gtk::Type::String];
        let dir_view  = gtk::TreeView::new();
        let dir_store = gtk::TreeStore::new(&column_types);

        let filename_column_num  = 0;
        let artist_column_num = 1;
        let album_column_num = 2;

        let filename_column  = self.get_new_column("Filename", filename_column_num);
        let artist_column = self.get_new_column("Artist", artist_column_num);
        let album_column  = self.get_new_column("Album", album_column_num);

        dir_view.append_column(&filename_column);
        dir_view.append_column(&artist_column);
        dir_view.append_column(&album_column);

        for ls in lsinfo {
            if ls.contains_key("directory") {
                let full_dirname = ls.get("directory").unwrap();
                let dirname = MainWindow::to_only_filename(full_dirname);

                let iter = dir_store.insert_with_values(None, None, &[0], &[&dirname]);

                self.get_inside_of_dir(full_dirname, &iter, &dir_store);
            } else if ls.contains_key("file") {
                let filename = MainWindow::to_only_filename(ls.get("file").unwrap()).to_value() as gtk::Value;

                let artist = match ls.get("Artist") {
                    None         => "".to_value() as gtk::Value,
                    Some(artist) => artist.to_value() as gtk::Value,
                };
                let album = match ls.get("Album") {
                    None         => "".to_value() as gtk::Value,
                    Some(album) => album.to_value() as gtk::Value,
                };

                let _ = dir_store.insert_with_values(None, None, &[0,1,2], &[&filename,&artist,&album]);
            }
        }

        dir_view.set_model(Some(&dir_store));
        // Search function is usefull so should also guess "true".
        dir_view.set_enable_search(false);
        dir_view.get_selection().set_mode(gtk::SelectionMode::Multiple);

        let locked_mpd = self.mpd.lock().unwrap();
        let mpd = locked_mpd.clone();
        dir_view.connect_key_press_event(move |widget,event_key| {
            // TODO: more better than that way.
            let mpd = mpd.clone();
            match event_key.get_keyval() as u32 {
                97 => {  // 97 means 'a'
                    let (paths, _) = widget.get_selection().get_selected_rows();
                    let model = widget.get_model().unwrap();
                    for path in paths {
                        let mut iter = model.iter_children(None).unwrap();
                        let mut pathname  = String::new();
                        let mut iter_path = String::new();
                        for index in path.get_indices() {
                            iter_path = match iter_path.as_ref() {
                                "" => index.to_string(),
                                _  => format!("{}:{}", iter_path, index.to_string()),
                            };

                            iter = model.get_iter_from_string(&iter_path).unwrap();

                            let currentname = model.get_value(&iter, 0).get::<String>().unwrap();
                            pathname = match pathname.as_ref() {
                                "" => currentname,
                                _  => format!("{}/{}", pathname, currentname),
                            };
                        }
                        mpd.add(&pathname);
                    }
                },
                _     => (),
            }
            gtk::prelude::Inhibit(false)
        });
        drop(locked_mpd);

        let scroll = gtk::ScrolledWindow::new(None, None);
        scroll.add(&dir_view);

        return scroll;
    }

    fn get_main_box(&self) -> gtk::Box {
        println!("main box");
        /* main Box */
        let primary_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);

        println!("Stack");
        /* Stack */
        let stack = gtk::Stack::new();
        stack.set_transition_type(gtk::StackTransitionType::SlideLeft);

        println!("StackSideBar");
        /* StackSidebar */
        let stack_sidebar = gtk::StackSidebar::new();
        stack_sidebar.set_stack(&stack);

        println!("Stack Inner");
        /* Stack inner */
        println!("Playlist");
        let pw = self.get_playlist_window();
        stack.add_titled(&pw, "playlist", "Playlist");

        println!("Album");
        let aw = self.get_album_window();
        stack.add_titled(&aw, "album", "Album");

        println!("MusicDirectory");
        let mw = self.get_music_dir_window();
        stack.add_titled(&mw, "music_dir", "MusicDirectory");

        println!("stack pack");
        primary_box.pack_start(&stack_sidebar, false, true, 5);
        primary_box.pack_start(&stack, true, true, 5);

        return primary_box;
    }
}

