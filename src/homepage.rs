extern crate reqwest;
extern crate tokio;

#[allow(dead_code)]
#[tokio::main]
pub async fn online() -> String {
    let content = reqwest::get("https://www.phoronix.com/").await.unwrap();
    assert!(content.status().is_success());
    let body = content.text().await.unwrap();
    body
}

#[allow(dead_code)]
pub fn offline() -> &'static str {
    include_str!("phoronix.html")
}
