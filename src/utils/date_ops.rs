use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

pub fn local_date() -> NaiveDateTime {
    Local::now().naive_local()
}

pub fn to_local_date(date: NaiveDateTime) -> String {
    Local
        .from_local_datetime(&date)
        .unwrap()
        .naive_local()
        .format("%v")
        .to_string()
}

pub fn from(timestamp: i64) -> NaiveDateTime {
    DateTime::from_timestamp(timestamp, 0)
        .unwrap()
        .naive_local()
}

pub fn to_timestamp() -> i64 {
    Local::now().timestamp()
}
