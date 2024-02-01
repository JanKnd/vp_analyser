use serde_derive::{Deserialize};
use std::fs;
use std::path::Path;
use std::process::exit;
use toml;
#[derive(Debug,Deserialize)]
pub struct Config {
    pub odd_timetable: String,
    pub even_timetable: String,
    pub monday: String,
    pub tuesday: String,
    pub wednesday: String,
    pub thursday: String,
    pub friday: String,
    pub does_update: bool
}
pub fn get_config() -> Config {
    let filename:&str = "vp_analyser_config.toml";
    let contents:String = fs::read_to_string(filename).unwrap();
    let config: Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", filename);
            exit(1);
        }
    };
    config
}