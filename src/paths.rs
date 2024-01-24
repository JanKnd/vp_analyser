use serde_derive::Deserialize;
use std::fs;
use std::path::Path;
use std::process::exit;
use toml;

#[derive(Debug,Deserialize)]
pub struct Paths{
    pub odd_timetable: String,
    pub even_timetable: String,
    pub monday: String,
    pub tuesday: String,
    pub wednesday: String,
    pub thursday: String,
    pub friday: String
}
pub fn get_paths() -> Paths{
    let filename:&str = "paths.toml";
    let contents:String = fs::read_to_string(filename).unwrap();
    let paths: Paths = match toml::from_str(&contents) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to load data from `{}`", filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };
    paths
}