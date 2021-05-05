use crate::article::Article;
use crate::homepage;
use crate::linesplit;
use term;

#[allow(dead_code)]
pub fn print() {
    let phoronix_articles = Article::get_articles(&homepage::online()); //online//
                                                                        // let phoronix_articles = Article::get_articles(homepage::offline()); //offline

    for article in phoronix_articles {
        println!(" Title: {}", article.title); //how to fix the numerous tabs and line breaks
        println!(" Link: https://phoronix.com/{}", article.link);
        println!(" Details: {}", article.details);
        println!(" Summary:");
        for line in linesplit::split_by_chars(&article.summary, 80) {
            println!(" - {}", line);
        }
        println!();
    }
}

pub fn print_coloured() {
    let phoronix_articles = Article::get_articles(&homepage::online());
    let mut terminal = term::stdout().unwrap();

    for article in phoronix_articles.iter().rev() {
        print!("Title:   ");
        terminal.fg(term::color::BRIGHT_YELLOW).unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
        println!("{}", article.title);
        terminal.reset().unwrap();

        print!("Link:    ");
        terminal.fg(term::color::BRIGHT_GREEN).unwrap();
        println!("https://www.phoronix.com/{}", article.link);
        terminal.reset().unwrap();

        print!("Details: ");
        terminal.fg(term::color::MAGENTA).unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
        println!("{}", article.details);
        terminal.reset().unwrap();

        print!("\nSummary:");
        for line in linesplit::split_by_chars(&article.summary, 77).iter() {
            print!(" - ");
            terminal.fg(term::color::BRIGHT_CYAN).unwrap();
            terminal.attr(term::Attr::Bold).unwrap();
            println!("{}", line);
            terminal.reset().unwrap();
        }
        println!("");
    }
}
