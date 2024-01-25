mod vp_parser;
mod vp_update;
mod timetable;
mod analyse;
mod paths;

use std::io::stdin;
use crate::vp_parser::Vertretungsplan;
use chrono::{TimeZone, Weekday};

fn main() {
   let paths: paths::Paths = paths::get_paths();
   /*
   let mut a: Vec<char> = vec![];
   a.push('a');
   a.push('b');
   a.push('c');
   */
   print!("{:?}",paths.does_update)//analyse::search_for_char_from_index_on(&a,&'d',Option::None))
}




