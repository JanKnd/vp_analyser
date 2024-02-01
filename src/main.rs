mod vp_parser;
mod vp_update;
mod timetable;
mod analyse;
mod vp_analyser_config;
use std::io::stdin;
use crate::vp_parser::Vertretungsplan;
use chrono::{TimeZone, Weekday};
lazy_static::lazy_static!{
   static ref CONFIG: vp_analyser_config::Config = vp_analyser_config::get_config();
   }
fn main() {
   /*
   let mut a: Vec<char> = vec![];
   a.push('a');
   a.push('b');
   a.push('c');
   */
   print!("{:?}",CONFIG.does_update);//analyse::search_for_char_from_index_on(&a,&'d',Option::None))
}




