use std::path::Path;
use glob::Pattern;
use chrono::{DateTime, Duration, NaiveDate, NaiveTime, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use crate::structs::Args;


pub fn watch_for_file(arguments: Args) -> i32 {
    let valid: bool = is_valid(arguments);
    if !valid {
        return 1
    }
    //TODO: WATCH FOR FILE

    0
}
fn is_valid(arguments: Args) -> bool{
    let path = Path::new(&arguments.path);
    if !path.is_dir() {
        return false
    }
    if !Pattern::new(&arguments.pattern).is_ok(){
        return false
    }
    if arguments.rundays >= 366 {
        return false
    }
    if !NaiveTime::parse_from_str(&arguments.endtime, "%H:%M:%S").is_ok() {
        return false
    }
    if let Some(val) = arguments.timezone {
        if !is_valid_timezone(val) {
            return false
        }
    }

    true
}
fn is_valid_timezone(timezone_string: &str)-> bool{
    timezone_string.parse::<Tz>().is_ok()
}

fn get_end_date_time(rundays: i32, endtime: &str, timezone: String) -> Result<DateTime<Tz>, chrono::ParseError> {
    let tz: Tz = timezone.parse().expect("Invalid timezone string");

    let now: DateTime<Tz> = tz.from_utc_datetime(&Utc::now().naive_utc());

    let future_date = now + Duration::days(rundays.into());

    let time = NaiveTime::parse_from_str(endtime, "%H:%M:%S")?;
    let date: NaiveDate = future_date.date_naive();
    let datetime = date.and_hms_opt(time.hour(), time.minute(), time.second()).unwrap();
    let future_datetime = tz.from_local_datetime(&datetime).unwrap();
    Ok(future_datetime)
}