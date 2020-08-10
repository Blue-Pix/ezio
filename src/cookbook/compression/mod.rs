use std::fs::File;
use std::path::PathBuf;
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use tar::{Archive, Builder};

type CustomResult = Result<(), std::io::Error>;

pub fn decompress_tarball() -> CustomResult {
  let path = "sample.tar.gz";
  let tar_gz = File::open(path)?;
  let tar = GzDecoder::new(tar_gz);
  let mut archive = Archive::new(tar);
  archive.unpack(".")?;
  Ok(())
}

pub fn compress_into_tarball() -> CustomResult {
  let tar_gz = File::create("archive.tar.gz")?;
  let enc = GzEncoder::new(tar_gz, Compression::default());
  let mut tar = Builder::new(enc);
  tar.append_dir_all("logs", "backup/logs")?;
  Ok(())
}

pub fn decompres_and_remove_prefix() {
  let file = File::open("archive.tar.gz").unwrap();
  let mut archive = Archive::new(GzDecoder::new(file));
  let prefix = "bundle/logs";
  println!("Extracted the following files:");
  archive
    .entries().unwrap()
    .filter_map(|e| e.ok())
    .map(|mut entry| -> Result<PathBuf, String> {
      let path = entry.path().unwrap().strip_prefix(prefix).unwrap().to_owned();
      entry.unpack(&path).unwrap();
      Ok(path)
    })
    .filter_map(|e| e.ok())
    .for_each(|x| println!("> {}", x.display()));
}