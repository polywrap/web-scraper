use polywrap_wasm_rs::Map;
use wrap::{*, imported::ArgsGet};
use scraper::{Html, Selector, ElementRef};
use imported::http_module::HttpModule;
use wrap::imported::{HttpResponseType, HttpRequest};

pub mod wrap;
pub use wrap::*;

fn extract_text(element: &ElementRef) -> String {
    element.text().collect::<Vec<_>>().concat()
}

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

        Ok(links.join(", ")) // Convert the Vec<String> into a single String
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

        // Update the selectors to target specific class
        let selectors = vec!["p", "h1"];

        let mut text = String::new();
        
        for selector in selectors {
            let selector = Selector::parse(selector).unwrap();
            for element in document.select(&selector) {
                text.push_str(&extract_text(&element));
                text.push_str(" ");
            }
        }
        
        Ok(text.trim().to_string()) // Trim leading and trailing whitespaces
    }
}
