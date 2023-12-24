use std::fmt::format;
use chrono::{Datelike, DateTime, Local, NaiveDate, Utc, Weekday};
use Option;
use std::path::Prefix::Verbatim;
use std::str;
use std::string::String;
use csv::{Reader, StringRecord};
use std::process::Command;
use execute::{command, Execute};

//Timetabel::new() returns right timetable for a and b week
#[derive(Debug)]
pub struct Timetable{
    pub montag: Vec<String>,
    pub dienstag: Vec<String>,
    pub mittwoch: Vec<String>,
    pub donnerstag: Vec<String>,
    pub freitag: Vec<String>
}

impl Timetable{
    pub fn new(path_to_csv: &str) -> Timetable{
        let weeknum:u32 = Local::now().iso_week().week();
        let mut path = String::new();
        if weeknum%2 != 0{
            path = "timetabel_odd.csv".to_string();
        } else {
            path = "timetabel_even.csv".to_string();
        }
        let tt_2dvec: Vec<Vec<String>> = record_vec_to_2dvec(read_csv(&path));
        Timetable{
            montag: tt_2dvec[0].clone(),
            dienstag: tt_2dvec[1].clone(),
            mittwoch: tt_2dvec[2].clone(),
            donnerstag: tt_2dvec[3].clone(),
            freitag: tt_2dvec[4].clone()
        }
    }
}

pub fn get_paths() -> [&'static str; 5] {
    let paths = ["VP/Montag/Montag S.pdf","VP/Dienstag/Dienstag S.pdf","VP/Mittwoch/Mittwoch S.pdf","VP/Donnerstag/Donnerstag S.pdf","VP/Donnerstag/Donnerstag S.pdf"];
    paths
}

pub fn get_dates() -> [String; 3] {
    let curr_date = Local::now().date_naive();
    let next_date = curr_date.succ_opt().unwrap();
    let next_next_date = next_date.succ_opt().unwrap();
    let formatted_dates = [format!("{}", curr_date.format("%d.%m.")),format!("{}", next_date.format("%d.%m.")),format!("{}", next_next_date.format("%d.%m."))];
    formatted_dates
}

pub fn get_weekdays() -> [String; 3] {
    let curr_day: NaiveDate = Local::now().date_naive();
    let next_day: NaiveDate = curr_day.succ_opt().unwrap();
    let next_next_day: NaiveDate = next_day.succ_opt().unwrap();
    return [curr_day.weekday().to_string(),next_day.weekday().to_string(),next_next_day.weekday().to_string()]
}

pub fn read_csv(path: &str) -> Vec<StringRecord>{
    let mut reader = csv::Reader::from_path(path).unwrap();

    let mut vec_results: Vec<StringRecord> = Vec::new();
    for result in reader.records(){
        let record = result.unwrap();

        vec_results.push(record);
    }
    vec_results
}

pub fn record_vec_to_2dvec(record_vec: Vec<StringRecord>) -> Vec<Vec<String>>{
    let mut result: Vec<Vec<String>> = Vec::new();
    let mut subvec: Vec<String> = Vec::new();

    for a in 1..6 {
        let mut temp: Vec<String> = Vec::new();
        for i in 0..11 {
            temp.push(record_vec[i].get(a).unwrap().to_string());
        }
        result.push(temp);
    }

    result
}

pub fn update_vp(){
    const MOODLE_DL_PATH: &str = "D:/Schule/VertretungsplanGrabber/VP-analyser";
    let mut update = Command::new(MOODLE_DL_PATH);
    update.arg("moodle-dl");
    print!("{:?}",update.execute_output().unwrap())
}
