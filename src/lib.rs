use wrap::{*, imported::ArgsGet};
use scraper::{Html, Selector, ElementRef};
use imported::http_module::HttpModule;
use wrap::imported::{HttpResponseType, HttpRequest};

pub mod wrap;
pub use wrap::*;

fn extract_text(element: &ElementRef) -> String {
    let text: String = element.text().collect::<Vec<_>>().join(" ");
    let text = text.replace("\n", " ").trim().to_string();
    text
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
        
        let document: Html = Html::parse_document(&result.unwrap().body.unwrap());

        let selectors = vec!["p", "h1", "h2", "h3", "h4", "h5", "h6", "div", "span"];

        let mut text_vec: Vec<String> = Vec::new();

        for selector in selectors {
            let selector = Selector::parse(selector).unwrap();
            for element in document.select(&selector) {
                let text = extract_text(&element);

                if !text.starts_with(".css") && !text.starts_with("html") {
                    text_vec.push(text);
                }
            }
        }

        Ok(text_vec.join(" ").trim().to_string())
    }
}
