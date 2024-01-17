mod pdf_parser;
mod vp_update;
mod timetable;
mod analyse;

use lopdf::Document;
use pdf_extract::OutputError;
use crate::pdf_parser::Vertretungsplaene;
use chrono::{Local, DateTime, TimeZone};
use crate::timetable::{get_dates, update_vp};


fn main() {
    update_vp()
}




