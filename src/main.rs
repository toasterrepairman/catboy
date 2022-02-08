use gtk::{TextBuffer, prelude::*, ResponseType};
use glib::{clone, MainContext};
use sourceview::*;
use std::env;
use std::path::Path;
use std::io::prelude::*;
use std::fs::*;
use clipboard::*;
use rfd::FileDialog;

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
    let statusbar: gtk::Statusbar = builder.get_object("statusbar").unwrap();
    let open: gtk::Button = builder.get_object("open").unwrap();
    let copy: gtk::Button = builder.get_object("copy").unwrap();

    // set up Sourceview buffer
    let cat_buf: TextBuffer = TextBuffer::new(None::<&gtk::TextTagTable>);
    text_output.set_buffer(Some(&cat_buf));

    // initialize arg vector
    let args: Vec<String> = env::args().collect();

    // printing function
    fn gui_print(path: &str, buffer: &TextBuffer) {
        let mut contents = String::new() ;
            File::open(Path::new(path))
                .expect("Unable to open file. Do you have proper permissions?")
                .read_to_string(&mut contents)
                    .expect("Unable to read the file");
        buffer.set_text(&contents)
    }

    // parse args
    match args.len() {
        1 => {
            println!("No args found, launching GUI.")
        }

        2 => {
            if Path::new(&args[1]).exists() {
                println!("found stuff!");
                gui_print(&args[1], &cat_buf);
            }
        }

        _ => println!("Input not parsable. If you're directly passing a string, try using quotes.")
    }

    // grab (optional) input from CLI
    // TODO

    // file opening handler (NON_FUNCTIONAL AS OF NOW)
    open.connect_clicked(move |_| {
        gui_print("~/.bashrc", &cat_buf);
    });

    /*
    // clipboard copy handler
    copy.connect_clicked(move |_| {
        // sets up keyboard context
        let mut ctx: ClipboardContext = ClipboardProvider::new()
            .expect("Failed to access clipboard");
        // pushes to clipboard by invoking textbuffer magicks
        // if anyone uses code like this in production, I refuse to let you blame me
        // seriously, converting a gstring to a String should not be this hard
        // why this garbage was not fixed prior to expanding GTK with libadwaita & friends is beyond me
        ctx.set_contents(cat_buf
                .get_text(&cat_buf.get_start_iter(), &cat_buf.get_end_iter(), true)
                .unwrap()
                .as_str()
                .to_string())
            .expect("Failed to write to clipboard");
    });
    */

    // window destructor (closes program properly)
    window.connect_destroy( |_| { 
        gtk::main_quit();
    });

    // init program
    window.show_all();
    gtk::main();
}

// :3