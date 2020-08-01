use std::env;

use ezio::simple_socket;
use ezio::custom_socket;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    // simple_socket::run(&args);
    custom_socket::run(&args);
}