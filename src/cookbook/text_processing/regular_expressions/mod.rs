use lazy_static::lazy_static;
use regex::{Regex, RegexSetBuilder};
use std::collections::HashSet;
use std::fmt;
use error_chain::error_chain;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::borrow::Cow;

error_chain! {
  foreign_links {
    Regex(regex::Error);
    Io(std::io::Error);
  }
}

pub fn validate_email() {
  assert_eq!(extract_login(r"I❤email@example.com"), Some(r"I❤email"));
  assert_eq!(extract_login(r"sdf+sdsfsd.as.sdsd@jhkk.d.rl"), Some(r"sdf+sdsfsd.as.sdsd"));
  assert_eq!(extract_login(r"More@Than@One@at.com"), None);
  assert_eq!(extract_login(r"Not and email@email"), None);
}

fn extract_login(input: &str) -> Option<&str> {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"(?x)
        ^(?P<login>[^@\s]+)@
        ([[:word:]]+\.)*
        [[:word:]]+$
        ").unwrap();
  }
  RE.captures(input).and_then(|cap| {
    cap.name("login").map(|login| login.as_str())
  })
}

pub fn hashtag() {
  let tweet = "Hey #world, I just got my new #dog, say hello to Till. #dog #forever #2 #_ ";
  let tags = extract_hashtags(tweet);
  println!("{:?}", tags);
}

fn extract_hashtags(text: &str) -> HashSet<&str> {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"\#[a-zA-Z][0-9a-zA-Z_]*").unwrap();
  }
  RE.find_iter(text).map(|mat| mat.as_str()).collect()
} 
 
pub fn phonenumber() -> Result<()> {
  let phone_text = "
    +1 505 881 9292 (v) +1 505 778 2212 (c) +1 505 881 9297 (f)
    (202) 991 9534
    Alex 5553920011
    1 (800) 233-2010
    1.299.339.1020";

  let re = Regex::new(
      r#"(?x)
        (?:\+?1)?                       # Country Code Optional
        [\s\.]?
        (([2-9]\d{2})|\(([2-9]\d{2})\)) # Area Code
        [\s\.\-]?
        ([2-9]\d{2})                    # Exchange Code
        [\s\.\-]?
        (\d{4})                         # Subscriber Number"#,
  )?;

  let phone_numbers = re.captures_iter(phone_text).filter_map(|cap| {
    let groups = (cap.get(2).or(cap.get(3)), cap.get(4), cap.get(5));
    match groups {
      (Some(area), Some(ext), Some(sub)) => Some(PhoneNumber {
        area: area.as_str(),
        exchange: ext.as_str(),
        subscriber: sub.as_str(),
      }),
      _ => None,
    }
  }).map(|m| m.to_string()).collect::<Vec<_>>();
  println!("{:?}", phone_numbers);
  Ok(())
}

struct PhoneNumber<'a> {
  area: &'a str,
  exchange: &'a str,
  subscriber: &'a str,
}

impl<'a> fmt::Display for PhoneNumber<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "1 ({}) {}-{}", self.area, self.exchange, self.subscriber)
  }
}

pub fn multiple_regex() -> Result<()> {
  let log_path = "application.log";
  let buffered = BufReader::new(File::open(log_path)?);
  let set = RegexSetBuilder::new(&[
    r#"version "\d.\d.\d""#,
    r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:443"#,
    r#"warning.*timeout expired"#,
  ]).case_insensitive(true).build()?;

  buffered.lines()
    .filter_map(|line| line.ok())
    .filter(|line| set.is_match(line.as_str()))
    .for_each(|x| println!("{}", x));

  Ok(())
}

pub fn replace() {
  let before = "2012-03-14, 2013-01-15 and 2014-07-05";
  let after = reformat_dates(before);
  println!("{}", before);
  println!("{}", after);
}

fn reformat_dates(before: &str) -> Cow<str> {
  lazy_static! {
    static ref ISO8601_DATE_REGEX: Regex = Regex::new(
      r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})"
    ).unwrap();
  }
  ISO8601_DATE_REGEX.replace_all(before, "$m/$d/$y")
}