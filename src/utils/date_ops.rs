use chrono::{Local, NaiveDate};

pub fn local_date() -> NaiveDate {
    Local::now().naive_local().date()
}
