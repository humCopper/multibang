use std::{env, fs, process};
use std::io::{Read, BufReader};
use std::error::Error;

#[derive(Debug)]
pub struct Bang(String, String);

#[derive(Debug)]
pub struct Config {
    pub bang: String,
    pub query: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // program name

        let bang = match args.next() {
            Some(arg) => arg,
            None => return Err("bang not found"),
        };

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("query not found"),
        };
        Ok(Config{ bang, query})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let url = url(&config.bang)
        .or(url_ddgo(&config.bang));
    if let Some(u) = url {
        let url = u.replace("{}", &config.query);
        let result = webbrowser::open(&url);
        if result.is_err() {
            eprintln!("Invalid Url: {}", url);
            process::exit(1);
        }
    } else {
        eprintln!("Bang {} is not found in the bangs file", config.bang);
        process::exit(1);
    }
    Ok(())
}

pub fn url(bang: &str) -> Option<String> {
    let mut file = fs::File::open("bangs").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let url = s.lines()
        .map(|line| -> Vec<&str> {
            line.split_whitespace().collect()})
        .find(|words| {
            words.get(0) == Some(&bang)
        })
        .and_then(|words|
            words.get(1)
                .map(|x| x.to_string()));
    url
}


pub fn url_ddgo(bang: &str) -> Option<String> {
    let file = fs::File::open("ddgo_bangs.json").unwrap();
    let reader = BufReader::new(file);
    let json: Vec<serde_json::Value>= serde_json::from_reader(reader).unwrap();
    let json: Vec<(String, String)> = json.iter().filter_map(|x| {
        match x {
            serde_json::Value::Object(y) => {
                Some((y.get("t").unwrap().as_str().unwrap().to_string(),
                          y.get("u").unwrap().as_str().unwrap().to_string()))
            }
            _ => None,
        }
    }).collect();
    let url = json.iter().find(|x| {
        x.0 == bang
    }).and_then(|bang| Some(bang.1.to_owned().replace("{{{s}}}", "{}")));
    url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ddgo_file() {
        let out = url_ddgo("alt");
        assert_eq!(out,Some("http://alternativeto.net/SearchResult.aspx?search={}".to_string()));
        let out = url_ddgo("5");
        assert_eq!(out,Some("http://fiverr.com/gigs/search?query={}".to_string()))

    }
}