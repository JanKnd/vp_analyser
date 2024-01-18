use chrono::Local;
use crate::vp_parser::Vertretungsplan;

pub fn get_dates_after_td(num_days: u8) -> Vec<char>{
    let mut curr_date = Local::now().date_naive();
    for _i in 0..num_days{
        curr_date = curr_date.succ_opt().unwrap();
    }
    let formatted_date = format!("{}",curr_date.format("%d.%m.")).chars().collect();
    formatted_date
}
pub fn search_for_char_from_index_on(text_file: &Vec<char>, character: &char, start_index: Option<usize>) -> Option<usize>{
    let mut start_index= start_index.unwrap_or(0);
    let text_size : usize = text_file.len();
    if start_index > text_size{
        return None;
    }
    for i in start_index..text_size{
        if &text_file[i] == character {
            return Some(i);
        }
    }
    return None;
}

/*
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
    return [NaiveDate::weekday(&curr_day).to_string(),next_day.weekday().to_string(),next_next_day.weekday().to_string()]
}

*/