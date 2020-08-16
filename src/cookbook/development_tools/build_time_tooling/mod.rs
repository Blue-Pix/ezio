use error_chain::error_chain;
use std::ffi::CString;
use std::os::raw::c_char;

error_chain! {
  foreign_links {
    NulError(::std::ffi::NulError);
    Io(::std::io::Error);
  }
}

fn prompt(s: &str) -> Result<String> {
  use std::io::Write;
  print!("{}", s);
  std::io::stdout().flush()?;
  let mut input = String::new();
  std::io::stdin().read_line(&mut input)?;
  Ok(input.trim().to_string())
}

extern {
  fn hello();
  fn greet(name: *const c_char);
  fn multiply(x: i32, y: i32) -> i32;
  fn print_app_info();
}

pub fn run_with_c() -> Result<()> {
  unsafe { hello() }
  let name = prompt("What's your name? ")?;
  let c_name = CString::new(name)?;
  unsafe { greet(c_name.as_ptr()) }
  Ok(())
}

pub fn run_with_cpp() {
  unsafe {
    println!("{}", multiply(5, 7));
  }
}

pub fn custom_define() {
  unsafe {
    print_app_info();
  }
}