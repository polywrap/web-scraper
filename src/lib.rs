use wrap::{*, imported::ArgsGet};
use scraper::{Html, Selector};
use imported::http_module::HttpModule;
use wrap::imported::{HttpResponseType, HttpRequest};

pub mod wrap;
pub use wrap::prelude::*;

impl ModuleTrait for Module {
    fn get_links(args: ArgsGetLinks) -> Result<Vec<String>, String> {
        let result = HttpModule::get(&ArgsGet {
            url: args.url.clone(),
            request: Some(HttpRequest{
                response_type: HttpResponseType::TEXT,
                headers: None,
                url_params: None,
                body: None,
                timeout: None,
                form_data: None,
            })
        })?;

        let document = Html::parse_document(&result.unwrap().body.unwrap());
        let selector = Selector::parse("a[href]").unwrap();

        let mut links = Vec::new();

        for link in document.select(&selector) {
            if let Some(href) = link.value().attr("href") {
                links.push(href.to_string());
            }
        }

        Ok(links)
    }

    fn get_text(args: ArgsGetText) -> Result<String, String> {
        let result = HttpModule::get(&ArgsGet {
            url: args.url.clone(),
            request: Some(HttpRequest{
                response_type: HttpResponseType::TEXT,
                headers: None,
                url_params: None,
                body: None,
                timeout: None,
                form_data: None,
            })
        })?;

        let document: Html = Html::parse_document(&result.unwrap().body.unwrap());
        let selector = Selector::parse("p,h1,h2,h3,h4,h5,h6,div,span").unwrap();

        let mut text: Vec<String> = vec!();

        for element in document.select(&selector) {
            for el in element.text().collect::<Vec<_>>() {
                if !el.starts_with(".css") && !el.starts_with("html") {
                    text.push(el.trim().to_string());
                }
            }
        }

        Ok(text.join("\n"))
    }
}
