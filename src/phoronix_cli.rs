use crate::article::Article;
use crate::homepage;

pub fn print() {
    let phoronix_articles = Article::get_articles(&homepage::online()); //online
    // let phoronix_articles = Article::get_articles(homepage::offline()); //offline

    for article in phoronix_articles {
        println!("Title: {}", article.title); //how to fix the numerous tabs and line breaks
        println!("Link: https://phoronix.com/{}", article.link);
        println!("Details: {}", article.details);
        println!("Summary: {}", article.summary);
        println!();
    }
}
