extern crate reqwest;
extern crate select;
extern crate tokio;

use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};

fn main() {
    let phoronix_articles = Article::get_articles();
    for article in phoronix_articles {
        println!("Title: {}", article.title); //how to fix the numerous tabs and line breaks
        println!("Link: https://phoronix.com/{}", article.link);
        println!("Details: {}", article.details);
        println!("Summary: {}", article.summary);
        println!();
    }
}

struct Article {
    title: String,
    link: String,
    details: String,
    summary: String,
}

fn open_doc() -> &'static str {
    include_str!("phoronix.html")
}

#[tokio::main]
async fn open_phoronix() -> String {
    let content = reqwest::get("https://www.phoronix.com/").await.unwrap();
    let body = content.text().await.unwrap();
    body
    // println!("the content -> {}", body); it works
}

impl Article {
    fn get_articles() -> Vec<Article> {
        Document::from(open_phoronix().as_str())
            .find(Name("article"))
            .into_iter() //no iter() method also found
            .map(|node| Article::new(&node))
            .collect()
    }
    fn new(node: &Node) -> Article {
        let header = node.find(Name("a")).next().unwrap(); //this had errors. No method named first
        let mut link = String::from(header.attr("href").unwrap());
        if link.starts_with("/") {
            assert_eq!(link.remove(0), '/');
        }

        let mut details = node.find(Class("details")).next().unwrap().text();
        if details.contains("Add A Comment") {
            details = details.replace("Add A Comment", "0 Comments")
        }

        let summary = node.find(Name("p")).next().unwrap().text();

        Article {
            title: header.text(),
            link: link,
            details: details,
            summary: summary,
        }
    }
}
