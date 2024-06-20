use chrono::{Local, NaiveDateTime};

pub fn local_date() -> NaiveDateTime {
    Local::now().naive_local()
}
