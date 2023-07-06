use reqwest;
use scraper::{Html, Selector};

pub struct ArgsGetText {
    pub url: String,
}

pub struct ArgsGetLinks {
    pub uri: String,
}

pub struct Module;

pub trait ModuleTrait {
    fn get_links(&self, args: ArgsGetLinks) -> Result<String, String>;
    fn get_text(&self, args: ArgsGetText) -> Result<String, String>;
}

impl ModuleTrait for Module {
    fn get_links(&self, args: ArgsGetLinks) -> Result<String, String> {
        let resp = reqwest::blocking::get(&args.uri)
            .map_err(|e| e.to_string())?
            .text()
            .map_err(|e| e.to_string())?;
        let document = Html::parse_document(&resp);
        let selector = Selector::parse("a[href]").unwrap();
        let links: Vec<String> = document.select(&selector)
                                         .filter_map(|link| link.value().attr("href"))
                                         .map(String::from)
                                         .collect();
        Ok(links.join(", "))
    }

    fn get_text(&self, args: ArgsGetText) -> Result<String, String> {
        let resp = reqwest::blocking::get(&args.url)
            .map_err(|e| e.to_string())?;
        resp.text().map_err(|e| e.to_string())
    }
}
