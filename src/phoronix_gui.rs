use crate::article::Article;
use crate::homepage;
use gdk::RGBA;
use gtk;
use gtk::traits::*;
use pango;

pub fn launch() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to init GTK"));

    //widgets for the articles
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0); 
    let articles = Article::get_articles(&homepage::online());
    generate_article_widgets(&container, &articles); //

    //insert the articles into a scrolled window
    let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scrolled_window.set_min_content_width(600);
    scrolled_window.add(&container);

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    build_ui(&window);
    window.add(&scrolled_window);
    window.show_all();

    // let escape = gdk::keyval_from_name("Escape");

    // window.connect_key_press_event(move |_, key| {
    //     match key {
    //          => gtk::main_quit(),
    //         _ => (),
    //     }
    //     gtk::Inhibit(false)
    // });

    gtk::main();
}

fn build_ui(window: &gtk::Window) {
    window.set_title("Phoronix Reader");
    window.set_default_size(600, 500);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        gtk::Inhibit(true)
    });
}

fn generate_article_widgets(container: &gtk::Box, articles: &Vec<Article>) {
    for article in articles {
        let url = format!("https://phoronix.com/{}", article.link);
        let title_and_url = gtk::LinkButton::with_label(&url, Some(&article.title)); //start adding unwrap here
        title_and_url.set_halign(gtk::Align::Start);

        let details = gtk::TextView::new(); //also here unwrap
        details.set_halign(gtk::Align::Start);
        details.set_left_margin(10);
        details.set_right_margin(10);
        details.set_editable(false);
        details.get_buffer().unwrap().set_text(&article.details); //You cannot set the text during the creation of the widget, so you
                                                                  //do so by using get_buffer method

        let summary = gtk::TextView::new();
        summary.set_wrap_mode(gtk::WrapMode::Word);
        summary.set_left_margin(10);
        summary.set_right_margin(10);
        summary.set_pixels_above_lines(10);
        summary.set_pixels_below_lines(10);
        summary.set_editable(false);
        summary.get_buffer().unwrap().set_text(&article.summary);

        container.add(&title_and_url);
        container.add(&details);
        container.add(&summary);
        container.add(&gtk::Separator::new(gtk::Orientation::Horizontal));
    }
}
