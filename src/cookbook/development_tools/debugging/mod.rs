// use error_chain::error_chain;
use std::env;
use env_logger::{Builder, Target};
use log::{Record, Metadata, Level, LevelFilter, SetLoggerError};
use syslog::{Facility, Error};
use chrono::Local;
use std::io::Write;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

// error_chain! {
//   foreign_links {
//       Io(std::io::Error);
//       LogConfig(log4rs::config::Errors);
//       SetLogger(log::SetLoggerError);
//   }
// }

fn execute_query(query: &str) {
  debug!("Executing query: {}", query);
}

pub fn debug() {
  // env::set_var("RUST_LOG", "debug");
  // env_logger::init();
  execute_query("DROP TABLE students");
}

fn execute_error_query(_query: &str) -> Result<(), &'static str> {
  Err("I'm afraid I can't do that")
}

pub fn error() {
  let response = execute_error_query("DROP TABLE students");
  if let Err(err) = response {
    error!("Failed to execute query: {}", err);
  }
}

pub fn stdout() {
  Builder::new()
           .target(Target::Stdout)
           .init();
  error!("This error has been printed to Stdout");
}

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= Level::Info
  }

  fn log(&self, record: &Record) {
    if self.enabled(record.metadata()) {
      println!("Rust says: {} - {}", record.level(), record.args());
    }
  }

  fn flush(&self) {}
}

pub fn custom_logger() -> Result<(), SetLoggerError> {
  log::set_logger(&CONSOLE_LOGGER)?;
  log::set_max_level(LevelFilter::Info);
  info!("hello log");
  warn!("warning");
  error!("oops");
  Ok(())
}

pub fn syslog() -> Result<(), Error> {
  syslog::init(
    Facility::LOG_USER,
    log::LevelFilter::Debug,
    Some("My app name")
  )?;
  debug!("this is a debug {}", "message");
  error!("this is an error!");
  Ok(())
}

mod foo {
  mod bar {
    pub fn run() {
      warn!("[bar] warn");
      info!("[bar] info");
      debug!("[bar] debug");
    }
  }

  pub fn run() {
    warn!("[foo] warn");
    info!("[foo] info");
    debug!("[foo] debug");
    bar::run();
  }
}

pub fn log_level() {
  env::set_var("RUST_LOG", "warn,ezio::cookbook::development_tools::debugging::foo=info,ezio::cookbook::development_tools::debugging::foo::bar=debug");
  env_logger::init();
  warn!("[root] warn");
  info!("[root] info");
  debug!("[root] debug");
  foo::run();
}

pub fn custom_environment() {
  env::set_var("MY_APP_LOG", "error");
  Builder::new()
           .parse_filters(&env::var("MY_APP_LOG").unwrap_or_default())
           .init();
  info!("informational message");
  warn!("warning message");
  error!("this is an error {}", "message");
}

pub fn with_timestamp() {
  Builder::new()
           .format(|buf, record| {
             writeln!(buf,
              "{} [{}] - {}",
              Local::now().format("%Y-%m-%dT%H:%M:%S"),
              record.level(),
              record.args()
            )
           })
           .filter(None, LevelFilter::Info)
           .init();
  warn!("warn");
  info!("info");
  debug!("debug");
}

pub fn custom_location() -> Result<(), String> {
  let logfile = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
    .build("log/output.log")
    .expect("Failed to build log file");

  let config = Config::builder()
    .appender(Appender::builder().build("logfile", Box::new(logfile)))
    .build(Root::builder()
                .appender("logfile")
                .build(LevelFilter::Info))
                .expect("Failed to build config");
  
  log4rs::init_config(config).expect("Failed to init config");
  info!("Hello, world!");
  Ok(())
}