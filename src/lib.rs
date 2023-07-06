use wasm_bindgen::prelude::*;
use scraper::{Html, Selector};
// ...

pub mod wrap;
pub use wrap::*;

impl ModuleTrait for Module {
    fn get_links(args: ArgsGetLinks) -> Result<String, String> {
        let document = Html::parse_document(&args.uri); 
        let selector = Selector::parse("a[href]").unwrap();

        let mut links = Vec::new();

        for link in document.select(&selector) {
            if let Some(href) = link.value().attr("href") {
                links.push(href.to_string());
            }
        }

        // Convert the Vec<String> into a single String to match the function signature
        Ok(links.join(", "))
    }

    fn get_text(args: ArgsGetText) -> Result<String, String> {
        let document = Html::parse_document(&args.url); // Changed from args.html to args.url
        let body = Selector::parse("body").unwrap();

        let mut text = String::new();

        for element in document.select(&body) {
            text.push_str(&element.inner_html());
        }

        Ok(text)
    }
}
