use std::{env, fs, process};
use std::io::Read;
use std::error::Error;

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

        let mut query = match args.next() {
            Some(arg) => arg,
            None => return Err("query not found"),
        };
        Ok(Config{ bang, query})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let url = url(&config.bang);
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
    file.read_to_string(&mut s);

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

//fn ddg_bangs() -> Result<serde_json::Value, Box<dyn Error>> {
//    let ddg_bangs = fs::File::open("ddg_bangs.json")?;
//    let json: serde_json::Value = serde_json::from_reader(ddg_bangs)?;
//    Ok(json)
//}
