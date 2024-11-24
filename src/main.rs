use reqwest::Result;
use scraper::{Html, Selector};

async fn get_quotes_page() -> Result<String> {
  let resp = reqwest::get("https://quotes.toscrape.com").await?;
  let data = resp.text().await?;
  Ok(data)
}

#[tokio::main]
async fn main() {
  let doc = Html::parse_document(get_quotes_page().await.unwrap().as_str());
  let selector = Selector::parse("div .quote .text").unwrap();
  let quotes = doc.select(&selector);

  for (idx, el) in quotes.enumerate() {
    let quote = el.text().collect::<Vec<&str>>();
    let count = idx + 1;
    println!("{count} {}", quote.join(""));
  }
}
