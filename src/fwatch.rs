use std::path::{Path, PathBuf};
use std::io;
use std::thread;
use std::time::Duration as StdDuration;
use glob::{glob, Pattern};
use chrono::{DateTime, Duration, NaiveDate, NaiveTime, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use crate::structs::Args;

pub fn watch_for_file(arguments: &Args) -> i32 {
    if !is_valid(arguments) {
        return 1;
    }

    let tz: Tz = arguments.timezone.parse().expect("validated by is_valid");

    let deadline = match get_end_date_time(arguments.rundays, &arguments.endtime, tz) {
        Ok(dt) => dt,
        Err(e) => {
            eprintln!("failed to compute end datetime: {}", e);
            return 1;
        }
    };

    let poll = StdDuration::from_secs(arguments.interval);
    eprintln!(
        "watching {} for `{}` until {} (every {}s)",
        arguments.path, arguments.pattern, deadline, arguments.interval
    );

    loop {
        match find_match(&arguments.path, &arguments.pattern) {
            Ok(Some(path)) => {
                println!("{}", path.display());
                return 0;
            }
            Ok(None) => {}
            Err(e) => {
                eprintln!("error scanning directory: {}", e);
                return 1;
            }
        }

        let now: DateTime<Tz> = tz.from_utc_datetime(&Utc::now().naive_utc());
        if now >= deadline {
            eprintln!("deadline reached, no match found");
            return 2;
        }
        thread::sleep(poll);
    }
}

fn find_match(dir_path: &str, glob_file: &str) -> Result<Option<PathBuf>, io::Error> {
    let mut full_path = PathBuf::from(dir_path);
    full_path.push(glob_file);

    let paths = glob(full_path.to_str().unwrap())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    for entry in paths {
        match entry {
            Ok(path) if path.is_file() => return Ok(Some(path)),
            Ok(_) => continue,
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    }
    Ok(None)
}

fn is_valid(arguments: &Args) -> bool {
    let path = Path::new(&arguments.path);
    if !path.is_dir() {
        eprintln!("path is not a directory: {}", arguments.path);
        return false;
    }
    if Pattern::new(&arguments.pattern).is_err() {
        eprintln!("invalid glob pattern: {}", arguments.pattern);
        return false;
    }
    if arguments.rundays < 0 || arguments.rundays >= 366 {
        eprintln!("rundays must be in 0..366, got {}", arguments.rundays);
        return false;
    }
    if NaiveTime::parse_from_str(&arguments.endtime, "%H:%M:%S").is_err() {
        eprintln!("endtime must be HH:MM:SS, got {}", arguments.endtime);
        return false;
    }
    if !is_valid_timezone(&arguments.timezone) {
        eprintln!("invalid timezone: {}", arguments.timezone);
        return false;
    }
    true
}

fn is_valid_timezone(timezone_string: &str) -> bool {
    timezone_string.parse::<Tz>().is_ok()
}

fn get_end_date_time(
    rundays: i32,
    endtime: &str,
    tz: Tz,
) -> Result<DateTime<Tz>, chrono::ParseError> {
    let now: DateTime<Tz> = tz.from_utc_datetime(&Utc::now().naive_utc());
    let future_date = now + Duration::days(rundays.into());
    let time = NaiveTime::parse_from_str(endtime, "%H:%M:%S")?;
    let date: NaiveDate = future_date.date_naive();
    let datetime = date
        .and_hms_opt(time.hour(), time.minute(), time.second())
        .expect("validated HMS components");
    let future_datetime = tz
        .from_local_datetime(&datetime)
        .single()
        .expect("non-ambiguous local datetime");
    Ok(future_datetime)
}
