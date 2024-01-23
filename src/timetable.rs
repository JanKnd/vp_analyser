use chrono::{Datelike, Local};
use Option;
use std::path::Prefix::Verbatim;
use std::str;
use std::string::String;
use csv::{Reader, StringRecord};

//Timetable::new() returns right timetable for a and b week
#[derive(Debug)]
pub struct Timetable{
    pub mon: Vec<Vec<char>>,
    pub tue: Vec<Vec<char>>,
    pub wed: Vec<Vec<char>>,
    pub thu: Vec<Vec<char>>,
    pub fri: Vec<Vec<char>>
}

impl Timetable{
    pub fn new(path_to_csv: &str) -> Timetable{
        let week_num:u32 = Local::now().iso_week().week();
        let mut path = String::new();
        if week_num%2 != 0{
            path = "timetabel_odd.csv".to_string();
        } else {
            path = "timetabel_even.csv".to_string();
        }
        let tt_2dvec: Vec<Vec<Vec<char>>> = record_vec_to_2dvec(read_csv(&path));
        Timetable{
            mon: tt_2dvec[0].clone(),
            tue: tt_2dvec[1].clone(),
            wed: tt_2dvec[2].clone(),
            thu: tt_2dvec[3].clone(),
            fri: tt_2dvec[4].clone()
        }
    }
}

pub fn get_paths() -> [&'static str; 5] {
    let paths = ["VP/Montag/Montag S.pdf","VP/Dienstag/Dienstag S.pdf","VP/Mittwoch/Mittwoch S.pdf","VP/Donnerstag/Donnerstag S.pdf","VP/Donnerstag/Donnerstag S.pdf"];
    paths
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

pub fn record_vec_to_2dvec(record_vec: Vec<StringRecord>) -> Vec<Vec<Vec<char>>>{
    let mut result: Vec<Vec<Vec<char>>> = Vec::new();


    for a in 1..6 {
        let mut temp: Vec<Vec<char>> = Vec::new();
        for i in 0..11 {
            temp.push(record_vec[i].get(a).unwrap().chars().collect());
        }
        result.push(temp);
    }

    result
}

