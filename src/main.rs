mod vp_parser;
mod vp_update;
mod timetable;
mod analyse;

use crate::vp_parser::Vertretungsplan;
use chrono::{TimeZone, Weekday};

fn main() {
   print!("{:?}",analyse::get_dates_after_td(0))
}




