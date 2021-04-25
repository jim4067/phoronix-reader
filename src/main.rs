extern crate reqwest;
extern crate select;
extern crate tokio;

mod article;
mod homepage;
mod linesplit;
mod phoronix_cli;

fn main() {
    phoronix_cli::print();
}
