use chrono::{Datelike, Local, Weekday};
#[derive(Debug)]


pub struct Vertretungsplan{
    pub vp: String,
}

impl Vertretungsplan{

    pub fn get_day(day: Weekday) -> Vertretungsplan{
        match day {
            Weekday::Mon => Vertretungsplan { vp: pdf_extract::extract_text("VP/Montag/Montag S.pdf").unwrap().to_string() },
            Weekday::Tue => Vertretungsplan {vp: pdf_extract::extract_text("VP/Dienstag/Dienstag S.pdf").unwrap().to_string()},
            Weekday::Wed => Vertretungsplan {vp: pdf_extract::extract_text("VP/Mittwoch/Mittwoch S.pdf").unwrap().to_string()},
            Weekday::Thu => Vertretungsplan {vp: pdf_extract::extract_text("VP/Donnerstag/Donnerstag S.pdf").unwrap().to_string()},
            Weekday::Fri => Vertretungsplan {vp: pdf_extract::extract_text("VP/Donnerstag/Donnerstag S.pdf").unwrap().to_string()},
            Weekday::Sat | Weekday::Sun => panic!("please enter a workday")
        }
    }
    pub fn get_today() -> Vertretungsplan {
        Vertretungsplan::get_day(Local::now().weekday())
    }
}