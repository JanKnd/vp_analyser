use chrono::{Datelike, Local, Weekday};
#[derive(Debug)]


pub struct Vertretungsplan{
    pub vp: Vec<char>,
}

impl Vertretungsplan{

    pub fn get_day(day: Weekday) -> Vertretungsplan{
        match day {
            Weekday::Mon => Vertretungsplan { vp: pdf_extract::extract_text("VP/Montag/Montag S.pdf").unwrap().chars().collect() },
            Weekday::Tue => Vertretungsplan {vp: pdf_extract::extract_text("VP/Dienstag/Dienstag S.pdf").unwrap().chars().collect()},
            Weekday::Wed => Vertretungsplan {vp: pdf_extract::extract_text("VP/Mittwoch/Mittwoch S.pdf").unwrap().chars().collect()},
            Weekday::Thu => Vertretungsplan {vp: pdf_extract::extract_text("VP/Donnerstag/Donnerstag S.pdf").unwrap().chars().collect()},
            Weekday::Fri => Vertretungsplan {vp: pdf_extract::extract_text("VP/Donnerstag/Donnerstag S.pdf").unwrap().chars().collect()},
            Weekday::Sat | Weekday::Sun => panic!("please enter a workday")
        }
    }
    pub fn get_today() -> Vertretungsplan {
        Vertretungsplan::get_day(Local::now().weekday())
    }
}