use gtk::{TextBuffer, prelude::*};
use glib::{clone, MainContext};
use sourceview::*;
use std::env;
use std::path::Path;
use std::io::prelude::*;
use std::fs::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // init builder
    let glade_src = include_str!("catboy.glade");
    let builder = gtk::Builder::from_string(glade_src);

    // init main window
    let window: gtk::Window = builder.get_object("mainwindow").unwrap();

    // init widgets (and mysterious sourceview incantation)
    sourceview::View::static_type();
    let text_output: gtk::TextView = builder.get_object("sourceview").unwrap();
    let filepicker: gtk::FileChooserDialog = builder.get_object("filepicker").unwrap();
    let statusbar: gtk::Statusbar = builder.get_object("statusbar").unwrap();
    let open: gtk::Button = builder.get_object("open").unwrap();
    let copy: gtk::Button = builder.get_object("copy").unwrap();

    // filepicker widgets
    let picker_cancel: gtk::Button = builder.get_object("picker_cancel").unwrap();
    let picker_open: gtk::Button = builder.get_object("picker_open").unwrap();

    // set up Sourceview buffer
    let cat_buf: gtk::TextBuffer = gtk::TextBuffer::new(None::<&gtk::TextTagTable>);
    text_output.set_buffer(Some(&cat_buf));

    // initialize arg vector
    let args: Vec<String> = env::args().collect();

    // parse args
    match args.len() {
        1 => {
            println!("No args found, launching GUI.")
        }

        2 => {
            if Path::new(&args[1]).exists() {
                println!("found stuff!");
                let mut contents = String::new() ;
                File::open(&args[1])
                    .expect("Unable to open file. Do you have proper permissions?")
                    .read_to_string(&mut contents).expect("Unable to read the file");
                println!("{}", contents);
            }
        }

        _ => println!("Input not parsable. If you're directly passing a string, try using quotes.")
    }

    // grab (optional) input from CLI
    // TODO

    // file opening handler (NON_FUNCTIONAL AS OF NOW)
    open.connect_clicked(move |_| {
        &filepicker.show();
    });


    // window destructor (closes program properly)
    window.connect_destroy( |_| { 
        gtk::main_quit();
    });

    // init program
    window.show_all();
    gtk::main();
}

// :3