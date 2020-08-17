use error_chain::error_chain;
use std::fs::File;
use std::io::Read;
use std::fmt;
use reqwest;
use serde::Deserialize;

error_chain!{
  foreign_links {
    Io(std::io::Error);
    ParseInt(std::num::ParseIntError);
    Reqwest(reqwest::Error);
    Reader(csv::Error);
  }
  errors { RandomResponseError(t: String) }
}

pub fn read_uptime() -> Result<u64> {
  let mut uptime = String::new();
  File::open("/proc/uptime")?.read_to_string(&mut uptime)?;
  Ok(uptime.split('.').next().ok_or("Cannot parse uptime data")?.parse()?)
}

fn request_random_number() -> Result<()> {
  let url = format!("https://www.random.org/integers/?num=1&min=0&max=10&col=1&base=10&format=plain");
  let response = reqwest::blocking::get(&url)?;
  let random_value: u32 = parse_response(response)?;
  println!("a random number between 0 and 10: {}", random_value);
  Ok(())
}

pub fn additional_error_kind(){
  if let Err(error) = request_random_number() {
    match *error.kind() {
        ErrorKind::Io(_) => println!("Standard IO error: {:?}", error),
        ErrorKind::Reqwest(_) => println!("Reqwest error: {:?}", error),
        ErrorKind::ParseInt(_) => println!("Standard parse int error: {:?}", error),
        ErrorKind::RandomResponseError(_) => println!("User defined error: {:?}", error),
        _ => println!("Other error: {:?}", error),
    }
  }
}

fn parse_response(response: reqwest::blocking::Response) -> Result<u32> {
  let mut body = response.text()?;
  body.pop();
  body.parse::<u32>().chain_err(|| ErrorKind::RandomResponseError(body))
}

pub fn create_rgb() -> Result<()> {
  let csv = "red,blue,green
102,256,204";
  let rgb = Rgb::from_reader(csv.as_bytes()).chain_err(|| "Cannot read CSV data")?;
  println!("{:?} to hexadecimal #{:X}", rgb, rgb);
  Ok(())
}

pub fn obtain_backtrace() {
  if let Err(ref errors) = create_rgb() {
    eprintln!("Error level - description");
    errors.iter().enumerate().for_each(|(index, error)| {
      eprintln!("└> {} - {}", index, error);
    });

    if let Some(backtrace) = errors.backtrace() {
      eprintln!("{:?}", backtrace);
    }
  }
}

#[derive(Debug, Deserialize)]
struct Rgb {
  red: u8,
  blue: u8,
  green: u8
}

impl Rgb {
  fn from_reader(csv_data: &[u8]) -> Result<Rgb> {
    let color: Rgb = csv::Reader::from_reader(csv_data)
        .deserialize()
        .nth(0)
        .ok_or("Cannot deserialize the first CSV record")?
        .chain_err(|| "Cannot deserialize RGB color")?;
    Ok(color)
  }
}

impl fmt::UpperHex for Rgb {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let hexa = u32::from(self.red) << 16 | u32::from(self.blue) << 8 | u32::from(self.green);
    write!(f, "{:X}", hexa)
  }
}