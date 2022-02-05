use gtk::{TextBuffer, prelude::*};
use glib::{clone, MainContext};
use sourceview::*;
use clap::Parser;

/// Concatenates input to a GTK
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input path of file to print
    #[clap(short, long, default_value = "")]
    filepath: String,

    /// Accept data from stdin to print
    #[clap(short, long, default_value = "")]
    raw: String,
}

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

    // init command line arguements
    let args = Args::parse();

    // grab (optional) input from CLI
    // TODO

    /*
    // file opening handler (NON_FUNCTIONAL AS OF NOW)
    open.connect_clicked(move |_| {
        &filepicker.show();
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