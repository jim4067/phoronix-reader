extern crate gdk;
extern crate gtk;
extern crate getopts;
extern crate pango;
extern crate reqwest;
extern crate select;
extern crate term;
extern crate tokio;

mod article;
mod homepage;
mod linesplit;
mod phoronix_cli;
mod phoronix_gui;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();

    opts.optflag("n", "no-color", "print without colors");
    opts.optflag("g", "gui", "open in a GTK GUI");
    opts.optflag("h", "help", "show this message");

    let matches = opts.parse(&args[1..]).unwrap();

    if matches.opt_present("h") {
        print_help();
        return;
    }
    match matches.opt_present("g") {
        true => phoronix_gui::launch(),
        false => {
            match matches.opt_present("n") {
                true => phoronix_cli::print(),
                false => phoronix_cli::print_coloured(),
            };
        }
    }

    // phoronix_cli::print_coloured();
}

fn print_help() {
    println!("Prints the latest headlines from Phoronix");
    println!("    -g, --gui       : launch in a GTK GUI window instead of outputting to the the terminal");
    println!("    -h, --help      : show this help message");
    println!("    -n, --no-color  : print without colors");
}
