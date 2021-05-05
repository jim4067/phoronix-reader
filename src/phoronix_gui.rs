use crate::article::Article;
use gdk::RGBA;
use gtk;
use gtk::traits::*;
use crate::homepage;
use pango;

pub fn launch() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to init GTK"));

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    configure_window(&window);

    window.show_all();

    gtk::main();
}

fn configure_window(window: &gtk::Window) {
    window.set_title("Phoronix Reader");
    window.set_default_size(600, 500);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        gtk::Inhibit(true)
    });
}
