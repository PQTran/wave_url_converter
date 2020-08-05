extern crate url;
extern crate colored;

use std::env;
use std::process;
use std::collections::HashMap;
use std::error::Error;
use url::Url;
use colored::Colorize;

fn format_url(id: &str) -> String {
    format!("https://wave.com.tw/plays/{}", id)
}

fn print_url(url: &str) {
    println!("Wave url: {}", url.purple().bold());
}

fn get_url_params(url: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let url = Url::parse(url)?;

    let pairs = url.query_pairs();
    let params_map: HashMap<String, String> = pairs.into_owned().collect();

    Ok(params_map)
}

fn get_input_argument(pos: usize) -> Result<String, Box<dyn Error>> {
    match env::args_os().nth(pos) {
        None => Err(From::from("Expected an argument, but received none.")),
        Some(input_str) => Ok(input_str.to_str().unwrap().to_owned())
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    const FIRST_ARG_INDEX: usize = 1;
    const ID_KEY: &str = "w_play";

    let wave_url = get_input_argument(FIRST_ARG_INDEX)?;
    let params_map = get_url_params(&wave_url)?;

    if let Some(id_val) = params_map.get(ID_KEY) {
        print_url(&format_url(id_val));
        Ok(())
    } else {
        Err(From::from(format!("Expected url to have param key: {}.", ID_KEY)))
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
