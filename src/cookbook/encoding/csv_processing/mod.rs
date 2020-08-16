use csv;
use serde::{Serialize, Deserialize, de, Deserializer};
use csv::{ReaderBuilder, Writer, Reader};
use std::str::FromStr;

pub fn read_csv() -> Result<(), csv::Error> {
  let csv = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";
  let mut reader = csv::Reader::from_reader(csv.as_bytes());
  for record in reader.records() {
    let record = record?;
    println!(
      "In {}, {} build the {} model. It is a {}",
      &record[0],
      &record[1],
      &record[2],
      &record[3],
    );
  }

  Ok(())
}

#[derive(Deserialize)]
struct Record {
  year: u16,
  make: String,
  model: String,
  description: String,
}

pub fn serde_deserialize() -> Result<(), csv::Error> {
  let csv = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";
  let mut reader = csv::Reader::from_reader(csv.as_bytes());
  for record in reader.deserialize() {
    let record: Record = record?;
    println!(
      "In {}, {} build the {} model. It is a {}",
      record.year,
      record.make,
      record.model,
      record.description,
    );
  }

  Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Record2 {
  name: String,
  place: String,
  #[serde(deserialize_with = "csv::invalid_option")]
  id: Option<u64>,
}

pub fn read_csv_with_different_delimiter() -> Result<(), csv::Error> {
  let data = "name\tplace\tid
  Mark\tMelbourne\t46
  Ashley\tZurich\t92";

  let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(data.as_bytes());
  for result in reader.deserialize::<Record2>() {
    println!("{:?}", result?);
  }
  
  Ok(())
}

pub fn filter_records() -> Result<(), String> {
  let query = "CA";
  let data = "\
  City,State,Population,Latitude,Longitude
  Kenai,AK,7610,60.5544444,-151.2583333
  Oakman,AL,,33.7133333,-87.3886111
  Sandfort,AL,,32.3380556,-85.2233333
  West Hollywood,CA,37031,34.0900000,-118.3608333";

  let mut reader = ReaderBuilder::new().from_reader(data.as_bytes());
  let mut writer = Writer::from_writer(std::io::stdout());
  writer.write_record(reader.headers().expect("Failed to get header")).expect("Failed to write record");

  for result in reader.records() {
    let record = result.expect("Failed to fetch record");
    if record.iter().any(|field| field == query) {
      writer.write_record(&record).expect("Failed to write record");
    }
  }

  writer.flush().expect("Failed to flush writer");
  Ok(())
}

pub fn handle_invalid_data() -> Result<(), csv::Error> {
  let data = "name,place,id
  mark,sydney,46.5
  ashley,zurich,92
  akshat,delhi,37
  alisha,colombo,xyz";

  let mut reader = Reader::from_reader(data.as_bytes());
  for result in reader.deserialize() {
    let record: Record2 = result?;
    println!("{:?}", record);
  }
  Ok(())
}

pub fn serialize_to_csv() -> Result<(), String> {
  let mut writer = Writer::from_writer(std::io::stdout());
  writer.write_record(&["Name", "Place", "ID"]).expect("Failed to writer record");
  writer.serialize(("Mark", "Sydney", 87)).expect("Failed to serialize record");
  writer.serialize(("Ashley", "Dublin", 32)).expect("Failed to serialize record");
  writer.serialize(("Akshat", "Dehli", 11)).expect("Failed to serialize record");
  writer.flush().expect("Faile to flush");
  Ok(())
}

pub fn serialize_with_serde() -> Result<(), String> {
  let mut writer = Writer::from_writer(std::io::stdout());
  let rec1 = Record2 { name: String::from("Mark"), place: String::from("Melbourne"), id: Some(56) };
  let rec2 = Record2 { name: String::from("Ashley"), place: String::from("Sydney"), id: Some(64) };
  let rec3 = Record2 { name: String::from("Akshat"), place: String::from("Delhi"), id: Some(98) };
  writer.serialize(rec1).expect("Failed to serialize");
  writer.serialize(rec2).expect("Failed to serialize");
  writer.serialize(rec3).expect("Failed to serialize");
  writer.flush().expect("Failed to flush");
  Ok(())
}

pub fn transform_csv_column() -> Result<(), String> {
  let data = "color_name,color
red,#ff0000
green,#00ff00
blue,#0000FF
periwinkle,#ccccff
magenta,#ff00ff".to_owned();

  let mut out = Writer::from_writer(vec![]);
  let mut reader = Reader::from_reader(data.as_bytes());
  for result in reader.deserialize::<Row>() {
    let res = result.expect("Failed to deserialize row");
    out.serialize((
      res.color_name,
      res.color.red,
      res.color.green,
      res.color.blue,
    )).expect("Failed to serialize row");
  }
  let written = String::from_utf8(out.into_inner().expect("Failed to into inner")).expect("Failed to convert from utf8");
  println!("{:?}", written.lines().last());
  println!("{}", written);
  Ok(())
}

#[derive(Debug)]
struct HexColor {
  red: u8,
  green: u8,
  blue: u8,
}

#[derive(Debug, Deserialize)]
struct Row {
  color_name: String,
  color: HexColor,
}

impl FromStr for HexColor {
  type Err = String;

  fn from_str(hex_color: &str) -> std::result::Result<Self, Self::Err> {
    let trimmed = hex_color.trim_matches('#');
    if trimmed.len() != 6 {
      Err("Invalid length of hex string".into())
    } else {
      Ok(HexColor {
        red: u8::from_str_radix(&trimmed[..2], 16).expect("Failed to parse int"),
        green: u8::from_str_radix(&trimmed[2..4], 16).expect("Failed to parse int"),
        blue: u8::from_str_radix(&trimmed[4..6], 16).expect("Failed to parse int"),
      })
    }
  }
}

impl<'de> Deserialize<'de> for HexColor {
  fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where D: Deserializer<'de>
  {
    let s = String::deserialize(deserializer)?;
    FromStr::from_str(&s).map_err(de::Error::custom)
  }
}