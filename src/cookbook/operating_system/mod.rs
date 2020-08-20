use regex::Regex;
use std::process::{Command, Stdio};
use error_chain::error_chain;
use std::io::{Write, BufRead, BufReader};
use std::collections::HashSet;
use std::fs::File;

error_chain! {
  foreign_links {
    Io(std::io::Error);
    Regex(regex::Error);
    Utf8(std::string::FromUtf8Error);
  }
}

pub fn exec_external_command() -> Result<()> {
  let output = Command::new("git").arg("log").arg("--oneline").output()?;
  if !output.status.success() {
    error_chain::bail!("Command executed with failing error code");
  }

  let pattern = Regex::new(r"(?x)
                             ([0-9a-fA-F]+) # commit hash
                             (.*)           # The commit message")?;
  String::from_utf8(output.stdout)?
    .lines()
    .filter_map(|line| pattern.captures(line))
    .map(|cap| {
      Commit {
        hash: cap[1].to_string(),
        message: cap[2].trim().to_string(),
      }
    })
    .take(5)
    .for_each(|x| println!("{:?}", x));

  Ok(())
}

#[derive(Debug, PartialEq, Default, Clone)]
struct Commit {
  hash: String,
  message: String
}

pub fn exec_python() -> Result<()> {
  let mut child = Command::new("python").stdin(Stdio::piped())
    .stderr(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;

  child.stdin
    .as_mut()
    .ok_or("Child process stdin has not been captured!")?
    .write_all(b"import this; copyright(); credits(); exit()")?;

  let output = child.wait_with_output()?;

  if output.status.success() {
    let raw_output = String::from_utf8(output.stdout)?;
    let words = raw_output.split_whitespace()
      .map(|s| s.to_lowercase())
      .collect::<HashSet<_>>();
    println!("Found {} unique words:", words.len());
    println!("{:#?}", words);
    Ok(())
  } else {
    let err = String::from_utf8(output.stderr)?;
    error_chain::bail!("External command failed:\n {}", err);
  }
}

pub fn pipe_external_command() -> Result<()> {
  let directory = std::env::current_dir()?;
  let mut du_output_child = Command::new("du")
    .arg("-ha")
    .arg(&directory)
    .stdout(Stdio::piped())
    .spawn()?;

  if let Some(du_output) = du_output_child.stdout.take() {
    let mut sort_output_child = Command::new("sort")
      .arg("-hr")
      .stdin(du_output)
      .stdout(Stdio::piped())
      .spawn()?;

    du_output_child.wait()?;

    if let Some(sort_output) = sort_output_child.stdout.take() {
      let head_output_child = Command::new("head")
        .args(&["-n", "10"])
        .stdin(sort_output)
        .stdout(Stdio::piped())
        .spawn()?;

      let head_output = head_output_child.wait_with_output()?;
      sort_output_child.wait()?;

      println!(
        "Top 10 biggest files and directories in '{}': \n{}",
        directory.display(),
        String::from_utf8(head_output.stdout)?
      )
    }
  }

  Ok(())
}

pub fn redirect_stdio() -> Result<()> {
  let outputs = File::create("out.txt")?;
  let errors = outputs.try_clone()?;
  Command::new("ls")
    .args(&[".", "oops"])
    .stdout(Stdio::from(outputs))
    .stderr(Stdio::from(errors))
    .spawn()?
    .wait_with_output()?;

  Ok(())
}

pub fn continous_process() -> Result<()> {
  let stdout = Command::new("cat")
    .arg("src/cookbook/operating_system/mod.rs")
    .stdout(Stdio::piped())
    .spawn()?
    .stdout
    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Error"))?;
  let reader = BufReader::new(stdout);
  reader.lines()
    .filter_map(|line| line.ok())
    .filter(|line| line.find("spawn").is_some())
    .for_each(|line| println!("{}", line));
  Ok(())
}

pub fn read_env() -> Result<()> {
  let config_path = std::env::var("CONFIG").unwrap_or("/etc/myapp/config".to_string());
  let config: String = std::fs::read_to_string(config_path)?;
  println!("Config: {}", config);
  Ok(())
}