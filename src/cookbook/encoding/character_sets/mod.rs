use percent_encoding::{utf8_percent_encode, percent_decode, AsciiSet, CONTROLS};
use std::str::Utf8Error;
use url::form_urlencoded::{byte_serialize, parse};
use data_encoding::{HEXUPPER, DecodeError};
use base64;
use std::str;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn percent_encoding() -> Result<(), Utf8Error> {
  let input = "confident, productive systems programming";
  let iter = utf8_percent_encode(input, FRAGMENT);
  let encoded: String = iter.collect();
  println!("{}", encoded);

  let iter = percent_decode(encoded.as_bytes());
  let decoded = iter.decode_utf8()?;
  println!("{}", decoded);
  Ok(())
}

pub fn form_urlencoded() {
  let urlencoded: String = byte_serialize("What is ❤?".as_bytes()).collect();
  println!("{}", urlencoded);
  let decoded: String = parse(urlencoded.as_bytes())
                        .map(|(key, val)| [key, val].concat())
                        .collect();
  println!("{}", decoded);
}

pub fn hex() -> Result<(), DecodeError> {
  let original = b"The quick brown fox jumps over the lazy dog.";
  let encoded = HEXUPPER.encode(original);
  println!("{}", encoded);
  let decoded = HEXUPPER.decode(&encoded.into_bytes())?;
  println!("{:?}", decoded);
  Ok(())
}

pub fn _base64() -> Result<(), String> {
  let hello = b"hello rustaceans";
  let encoded = base64::encode(hello);
  let decoded = base64::decode(&encoded).expect("Failed to decode base64");
  println!("origin: {}", str::from_utf8(hello).expect("Failed to print origin"));
  println!("base64 encoded: {}", encoded);
  println!("back to origin: {}", str::from_utf8(&decoded).expect("Failed to decode base64"));
  Ok(())
}