use serde_json::json;
use toml;
use serde::Deserialize;
use std::collections::HashMap;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub fn parse_json() -> Result<(), serde_json::Error> {
  let j = r#"{
              "userid": 103609,
              "verified": true,
              "access_privileges": [
                "user",
                "admin"
              ]
            }"#;
  let parsed: serde_json::Value = serde_json::from_str(j)?;
  let expected = json!({
    "userid": 103609,
    "verified": true,
    "access_privileges": [
      "user",
      "admin"
    ]
  });
  println!("{:?}", parsed);
  assert_eq!(parsed, expected);
  Ok(())
}

pub fn parse_toml() -> Result<(), toml::de::Error> {
  let toml_content = r#"
    [package]
    name = "your_package"
    version = "0.1.0"
    authors = ["You! <you@example.org>"]

    [dependencies]
    serde = "1.0"
  "#;
  let package_info: toml::Value = toml::from_str(toml_content)?;
  println!("{:?}", package_info);
  assert_eq!(package_info["dependencies"]["serde"].as_str(), Some("1.0"));
  assert_eq!(package_info["package"]["name"].as_str(), Some("your_package"));
  Ok(())
}

pub fn parse_toml_into_struct() -> Result<(), toml::de::Error> {
  let toml_content = r#"
          [package]
          name = "your_package"
          version = "0.1.0"
          authors = ["You! <you@example.org>"]

          [dependencies]
          serde = "1.0"
          "#;
  let package_info: Config = toml::from_str(toml_content)?;
  println!("{}", package_info.package.name);
  println!("{}", package_info.package.version);
  println!("{:?}", package_info.package.authors);
  println!("{}", package_info.dependencies["serde"]);
  Ok(())
}

#[derive(Deserialize)]
struct Config {
  package: Package,
  dependencies: HashMap<String, String>
}

#[derive(Deserialize, Debug)]
struct Package {
  name: String,
  version: String,
  authors: Vec<String>
}

pub fn little_endian() -> Result<(), std::io::Error> {
  let original_payload = Payload {
    kind: 129,
    value: 29485,
  };
  let encoded_bytes = encode(&original_payload)?;
  let decoded_payload = decode(&encoded_bytes)?;
  println!("{:?}", original_payload);
  println!("{:?}", encoded_bytes);
  println!("{:?}", decoded_payload);
  assert_eq!(original_payload, decoded_payload);
  Ok(())
}

#[derive(Default, PartialEq, Debug)]
struct Payload {
  kind: u8,
  value: u16,
}

fn encode(payload: &Payload) -> Result<Vec<u8>, std::io::Error> {
  let mut bytes = vec![];
  bytes.write_u8(payload.kind)?;
  bytes.write_u16::<LittleEndian>(payload.value)?;
  Ok(bytes)
}

fn decode(mut bytes: &[u8]) -> Result<Payload, std::io::Error> {
  let payload = Payload {
    kind: bytes.read_u8()?,
    value: bytes.read_u16::<LittleEndian>()?,
  };
  Ok(payload)
}