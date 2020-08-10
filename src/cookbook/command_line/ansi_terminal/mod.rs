use ansi_term::{Colour, Style};

pub fn print_colored_text() {
  println!("This is {} in color, {} in color and {} in color",
          Colour::Red.paint("red"),
          Colour::Blue.paint("blue"),
          Colour::Green.paint("green"));
}

pub fn print_bold_text() {
  println!("{} and this is not", 
          Style::new().bold().paint("This is Bold"));
}

pub fn print_bold_and_coloured_text() {
  println!("{}, {} and {}",
          Colour::Yellow.paint("This is coloured"),
          Style::new().bold().paint("this is bold"),
          Colour::Yellow.bold().paint("this is bold and coloured"));
}