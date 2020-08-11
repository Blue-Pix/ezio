extern crate failure;
use data_encoding::HEXUPPER;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use ring::digest::{Digest, Context, SHA256};
use ring::{hmac, rand};
use ring::rand::SecureRandom;
use ring::error::Unspecified;

pub fn calculate_sha256() -> Result<(), failure::Error> {
  let path = "file.txt";
  let mut f = File::create(path)?;
  write!(f, "We will generate a digest of this text.")?;

  let input = File::open(path)?;
  let reader = BufReader::new(input);
  let digest = sha256_digest(reader)?;
  println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()));
  Ok(())
}

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, failure::Error> {
  let mut context = Context::new(&SHA256);
  let mut buffer = [0; 1024];
  loop {
    let count = reader.read(&mut buffer)?;
    if count == 0 {
      break;
    }
    context.update(&buffer[..count]);
  }
  Ok(context.finish())
}

pub fn verify_sign() -> Result<(), Unspecified> {
  let mut key_value = [0u8; 48];
  let rng = rand::SystemRandom::new();
  rng.fill(&mut key_value)?;
  let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);
  let message = "Legitimate and important message";
  let signature = hmac::sign(&key, message.as_bytes());
  hmac::verify(&key, message.as_bytes(), signature.as_ref())?;
  Ok(())
}