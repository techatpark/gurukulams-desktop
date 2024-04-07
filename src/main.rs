use gtk::{gio, glib, prelude::*, ApplicationWindow, Builder};

use std::fs::File;
use std::io::{BufReader, Read as _};
use std::path::PathBuf;

use sourceview5::prelude::*;

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("window.glade");
    let builder = Builder::new();
    builder
        .add_from_string(glade_src)
        .expect("Builder couldn't add from string");

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");

    window.set_application(Some(application));
    window.set_title(Some("Gurukulams"));
    window.set_default_size(500, 500);

    

    let buffer = sourceview5::Buffer::new(None);
    buffer.set_highlight_syntax(true);
    if let Some(ref language) = sourceview5::LanguageManager::new().language("markdown") {
        buffer.set_language(Some(language));
    }
    if let Some(ref scheme) = sourceview5::StyleSchemeManager::new().scheme("solarized-light") {
        buffer.set_style_scheme(Some(scheme));
    }

    let open_button: gtk::Button = builder.object("open_button").unwrap();
    let file_open: gtk::FileChooserDialog = builder.object("file_open").unwrap();
    file_open.add_buttons(&[
        ("Open", gtk::ResponseType::Ok.into()),
        ("Cancel", gtk::ResponseType::Cancel.into()),
    ]);

    // Connect to "clicked" signal of `button`
    open_button.connect_clicked(move |_| {
        file_open.show();
        file_open.connect_response(move |d, response| {
            if response == gtk::ResponseType::Ok {
                let selected_file = d.file().expect("Couldn't get filename");
    
                println!("loaded: {:?}", &open_file(selected_file.path()));
            }
            
            d.destroy();
        });
    });

    let file = gio::File::for_path("./README.md");
    let file = sourceview5::File::builder().location(&file).build();
    let loader = sourceview5::FileLoader::new(&buffer, &file);
    loader.load_async_with_callback(
        glib::Priority::default(),
        gio::Cancellable::NONE,
        move |current_num_bytes, total_num_bytes| {
            println!(
                "loading: {:?}",
                (current_num_bytes as f32 / total_num_bytes as f32) * 100f32
            );
        },
        |res| {
            println!("loaded: {:?}", res);
        },
    );

    let editor_container = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let glade_src = include_str!("md_toolbar.glade");
    let builder = Builder::new();
    builder
        .add_from_string(glade_src)
        .expect("Builder couldn't add from string");

    let md_toolbar: gtk::Box = builder.object("md_toolbar").expect("Couldn't get window");

    editor_container.append(&md_toolbar);

    let view = sourceview5::View::with_buffer(&buffer);
    view.set_monospace(true);
    view.set_background_pattern(sourceview5::BackgroundPatternType::Grid);
    view.set_show_line_numbers(true);
    view.set_highlight_current_line(true);
    view.set_tab_width(4);
    view.set_hexpand(true);
    editor_container.append(&view);

    // let map = sourceview5::Map::new();
    // map.set_view(&view);
    // container.append(&map);

    window.set_child(Some(&editor_container));
    window.show();
}

fn open_file(filename: Option<PathBuf>) -> String {
    match filename {
        Some(p) => {
            let file = File::open(p).expect("Couldn't open file");
            let mut reader = BufReader::new(file);
            let mut contents = String::new();
            let _ = reader.read_to_string(&mut contents);

            contents
        },
        None => "".to_owned()
    }
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.bilelmoussaoui.sourceview5-example"),
        Default::default(),
    );
    application.connect_activate(build_ui);

    application.run();
}
