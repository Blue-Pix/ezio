use std::time::{Duration, Instant};
use std::thread;
use chrono::{self, DateTime, Utc, Local, FixedOffset};

fn expensive_function() {
  thread::sleep(Duration::from_secs(3));
}

pub fn elapsed_time() {
  let start = Instant::now();
  expensive_function();
  let duration = start.elapsed();
  println!("Time elapsed in expensive_function() is: {:?}", duration);
}

pub fn calculate_time() {
  let now = Utc::now();
  println!("{}", now);

  let three_weeks_from_now = now.checked_add_signed(chrono::Duration::weeks(2))
                                .and_then(|in_2weeks| in_2weeks.checked_add_signed(chrono::Duration::weeks(1)))
                                .and_then(day_earlier);
  match three_weeks_from_now {
    Some(x) => println!("{}", x),
    None => eprintln!("Almost three weeks from now overflows!"),
  }

  match now.checked_add_signed(chrono::Duration::max_value()) {
    Some(x) => println!("{}", x),
    None => eprintln!("We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center.")
  }
}

fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
  date_time.checked_sub_signed(chrono::Duration::days(1))
}

pub fn local_timezone() {
  let local_time = Local::now();
  let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
  let china_timezone = FixedOffset::east(8 * 3600);
  let rio_timezone = FixedOffset::west(2 * 3600);
  println!("Local time now is {}", local_time);
  println!("UTC time now is {}", utc_time);
  println!("Time in Hong Kong now is {}", utc_time.with_timezone(&china_timezone));
  println!("Time in Rio de Janeiro now is {}", utc_time.with_timezone(&rio_timezone));
}