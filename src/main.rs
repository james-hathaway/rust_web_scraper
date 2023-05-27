use scraper::{Html, Selector};
use clap::{App, Arg};
use rust_web_scraper::WebScraper;

async fn main() -> Result<(), Box<dyn, std::error::Error>> {

    let matches = App::new("Web Scraper").version("1.0")
    .arg(Arg::with_name("urls").mulitple(true).required(true))
    .get_matches(true);

    let urls: Vec<_> = matches.values_of("urls").unwrap().collect();

    let scraper = WebScraper::new(urls);
    let bodies = scraper.scrape_multiple().await?

    for body in bodies {
        println!("{}", scraper.analyze(&body));
    }

    Ok(())

    
}

pub struct WebScraper {
    urls: String,
}

impl WebScraper {
    
    pub fn new(urls: Vec<String>) -> WebScraper {
        WebScraper { urls }
    }

    pub async fn scrape (&self, url: &str) -> Result<String, Box<dyn std::error:Error>> {
        let some = reqwest::get(url).await?.text().await?
        Ok(some)
    }

    pub async fn scrape_multiple(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let bodies = stream::iter(self.urls.clone())
        .map(|url| self.scrape(&url))
        .buffer_unordered(self.urls.len())
        .collect::Result<Vec<_>, >>().await?

        Ok(bodies)
    }

    pub async fn analyze(&self, body: &str) -> usize {
        let fragment = Html::parse_fragment(body);
        let sector = Selector::("parse").unwrap();

        fragment.select(&Selector).count()
    }

}