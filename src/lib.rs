use polywrap_wasm_rs::Map;
use wrap::{*, imported::ArgsGet};
use scraper::{Html, Selector};
use imported::http_module::HttpModule;
use wrap::imported::{HttpResponseType, HttpRequest};

pub mod wrap;
pub use wrap::*;

impl ModuleTrait for Module {
    fn get_links(args: ArgsGetLinks) -> Result<String, String> {
        let result = HttpModule::get(&ArgsGet {
            url: args.uri.clone(),
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

        // Convert the Vec<String> into a single String to match the function signature
        Ok(links.join(", "))
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
        
        let document = Html::parse_document(&result.unwrap().body.unwrap());
        let body = Selector::parse("body").unwrap();

        let mut text = String::new();

        for element in document.select(&body) {
            text.push_str(&element.inner_html());
        }

        Ok(text)
    }
}