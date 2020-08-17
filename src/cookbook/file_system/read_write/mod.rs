use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use same_file::Handle;
use std::path::Path;
use memmap::Mmap;

pub fn read_lines() -> Result<(), std::io::Error> {
  let path = "lines.txt";
  let mut output = File::create(path)?;
  write!(output, "Rust\n💖\nFun")?;

  let input = File::open(path)?;
  let bufread = BufReader::new(input);
  for line in bufread.lines() {
    println!("{}", line?);
  }
  Ok(())
}

pub fn handle_same_file() -> Result<(), std::io::Error> {
  let path_to_read = Path::new("new.txt");
  let stdout_handle = Handle::stdout()?;
  let handle = Handle::from_path(path_to_read)?;
  if stdout_handle == handle {
    println!("You are reading and writing to the same file");
    return Err(std::io::Error::new(
      std::io::ErrorKind::Other,
      "You are reading and writing to the same file",
    ));
  } else {
    let file = File::open(&path_to_read)?;
    let file = BufReader::new(file);
    for (num, line) in file.lines().enumerate() {
      println!("{} : {}", num, line?.to_uppercase());
    }
  }
  Ok(())
}

pub fn access_file_randomly() -> Result<(), std::io::Error> {
  write!(File::create("content.txt")?, "My hovercraft is full of eels!")?;
  let file = File::open("content.txt")?;
  let map = unsafe { Mmap::map(&file)? };
  let random_indexes = [0, 1, 2, 19, 22, 10, 11, 29];
  println!("{:?}", &map[3..13]);
  assert_eq!(&map[3..13], b"hovercraft");
  let random_bytes: Vec<u8> = random_indexes.iter()
    .map(|&idx| map[idx])
    .collect();
  println!("{:?}", random_bytes);
  assert_eq!(random_bytes, b"My loaf!");
  Ok(())
}