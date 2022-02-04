use gtk::{TextBuffer, prelude::*};
use glib::{clone, MainContext};
use sourceview::*;

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

    let cat_buf: gtk::TextBuffer = gtk::TextBuffer::new(None::<&gtk::TextTagTable>);
    text_output.set_buffer(Some(&cat_buf));

    // window destructor (closes program properly)
    window.connect_destroy( |_| { 
        gtk::main_quit();
    });

    // init program
    window.show_all();
    gtk::main();
}

// :3