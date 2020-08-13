use chrono::{Datelike, Timelike, Utc, NaiveDate, NaiveDateTime, NaiveTime, DateTime};
use chrono::format::ParseError;

pub fn examine_date_and_time() {
  let now = Utc::now();
  let (is_pm, hour) = now.hour12();
  println!(
    "The current UTC time is {:02}:{:02}:{:02} {}",
    hour,
    now.minute(),
    now.second(),
    if is_pm { "PM" } else { "AM" }
  );

  let (is_common_era, year) = now.year_ce();
  println!(
    "The current UTC date is {}-{:02}-{:02} {:?} ({})",
    year,
    now.month(),
    now.day(),
    now.weekday(),
    if is_common_era { "CE" } else { "BCE" }
  );
  println!(
    "And the Common Era began {} days ago",
    now.num_days_from_ce()
  );
}

pub fn unix_time() {
  let date_time: NaiveDateTime = NaiveDate::from_ymd(2020, 8, 12).and_hms(19, 36, 1);
  println!(
    "Number of seconds between 1970-01-01 00:00:00 and {} is {}.",
    date_time, date_time.timestamp()
  );

  let date_time_after_a_billion_seconds = NaiveDateTime::from_timestamp(1_000_000_000, 0);
  println!(
    "Date after a billion seconds since 1970-01-01 00:00:00 was {}",
    date_time_after_a_billion_seconds
  );
}

pub fn format() {
  let now = Utc::now();
  println!("UTC now is {}", now);
  println!("UTC now in RFC2822 is: {}", now.to_rfc2822());
  println!("UTC now in RFC3339 is: {}", now.to_rfc3339());
  println!("UTC now in a custom format is: {}", now.format("%a %b %e %T %Y"));
}

pub fn parse() -> Result<(), ParseError> {
  let rfc2822 = DateTime::parse_from_rfc2822("Thu, 13 Aug 2020 18:17:05 +0900")?;
  println!("{}", rfc2822);
  let rfc3339 = DateTime::parse_from_rfc3339("2020-08-12T18:17:05+09:00")?;
  println!("{}", rfc3339);
  let custom = DateTime::parse_from_str("13.8.2020 18:00 pm +0900", "%d.%m.%Y %H:%M %P %z")?;
  println!("{}", custom);
  let time_only = NaiveTime::parse_from_str("18:17:05", "%H:%M:%S")?;
  println!("{}", time_only);
  let date_only = NaiveDate::parse_from_str("2020-08-13", "%Y-%m-%d")?;
  println!("{}", date_only);
  let no_timezone = NaiveDateTime::parse_from_str("2020-08-13 18:17:05", "%Y-%m-%d %H:%M:%S")?;
  println!("{}", no_timezone);
  Ok(())
}