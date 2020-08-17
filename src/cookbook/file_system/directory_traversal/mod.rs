use error_chain::error_chain;
use std::{env, fs};
use std::path::{Path, PathBuf};
use same_file::is_same_file;
use std::collections::HashMap;
use walkdir::{WalkDir, DirEntry};
use glob::glob;
use glob::MatchOptions;
use glob::glob_with;

error_chain! {
  foreign_links {
    Io(std::io::Error);
    SystemTimeError(std::time::SystemTimeError);
    WalkDir(walkdir::Error);
    Glob(glob::GlobError);
    Pattern(glob::PatternError);
  }
}

pub fn last_modified_within_a_day() -> Result<()> {
  let current_dir = env::current_dir()?;
  println!("Entries modified in the last 24 hours in {:?}:", current_dir);
  for entry in fs::read_dir(current_dir)? {
    let entry = entry?;
    let path = entry.path();
    let metadata = fs::metadata(&path)?;
    let last_modified = metadata.modified()?.elapsed()?.as_secs();
    if last_modified < 24 * 3600 && metadata.is_file() {
      println!(
        "Last modified: {:?} seconds, is read ony: {:?}, size: {:?} bytes, filename: {:?}",
        last_modified,
        metadata.permissions().readonly(),
        metadata.len(),
        path.file_name().ok_or("No filename")?
      )
    }
  }
  Ok(())
}

pub fn find_loop() {
  println!("{:?}", contains_loop("/tmp/foo/bar/qux/bar/baz").unwrap());
}

fn contains_loop<P: AsRef<Path>>(path: P) -> std::io::Result<Option<(PathBuf, PathBuf)>> {
  let path = path.as_ref();
  let mut path_buf = path.to_path_buf();
  while path_buf.pop() {
    if is_same_file(&path_buf, path)? {
      return Ok(Some((path_buf, path.to_path_buf())));
    } else if let Some(looped_paths) = contains_loop(&path_buf)? {
      return Ok(Some(looped_paths));
    }
  }
  return Ok(None)
}

pub fn find_duplicate_file_names() {
  let mut filenames = HashMap::new();
  for entry in WalkDir::new(".")
    .into_iter()
    .map(|e| e.unwrap())
    .filter(|e| !e.file_type().is_dir()) {
      let f_name = String::from(entry.file_name().to_string_lossy());
      let counter = filenames.entry(f_name.clone()).or_insert(0);
      *counter += 1;
      if *counter == 2 {
        println!("{}", f_name);
      }
  }
}

pub fn resursive_find_all() -> Result<()> {
  for entry in WalkDir::new(".")
    .follow_links(true)
    .into_iter()
    .filter_map(|e| e.ok()) {
      let f_name = entry.file_name().to_string_lossy();
      let sec = entry.metadata()?.modified()?;

      if f_name.ends_with(".json") && sec.elapsed()?.as_secs() < 60 * 24 {
        println!("{}", f_name);
      }
    }
  Ok(())
}

pub fn skip_hidden_files() {
  WalkDir::new(".")
    .into_iter()
    .filter_entry(|e| is_not_hidden(e))
    .filter_map(|v| v.ok())
    .for_each(|x| println!("{}", x.path().display()));
}

fn is_not_hidden(entry: &DirEntry) -> bool {
  entry
    .file_name()
    .to_str()
    .map(|s| entry.depth() == 0 || !s.starts_with("."))
    .unwrap_or(false)
}

pub fn calc_dir_size() {
  let total_size = WalkDir::new(".")
    .min_depth(1)
    .max_depth(3)
    .into_iter()
    .filter_map(|entry| entry.ok())
    .filter_map(|entry| entry.metadata().ok())
    .filter(|metadata| metadata.is_file())
    .fold(0, |acc, m| acc + m.len());
  println!("Total size: {} bytes.", total_size);
}

pub fn find_png_recursively() -> Result<()> {
  for entry in glob("**/*.png")? {
    println!("{}", entry?.display());
  }
  Ok(())
}

pub fn glob_with_option() -> Result<()> {
  let options = MatchOptions {
    case_sensitive: false,
    ..Default::default()
  };
  for entry in glob_with("/media/img_[0-9]*.png", options)? {
    println!("{}", entry?.display());
  }
  Ok(())
}