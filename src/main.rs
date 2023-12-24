mod pdf_parser;
mod vp_update;
mod data;

use lopdf::Document;
use pdf_extract::OutputError;
use crate::pdf_parser::Vertretungsplaene;
use chrono::{Local, DateTime, TimeZone};
use crate::data::{get_dates, update_vp};


fn main() {
    update_vp()
}




