use std::env;

#[allow(unused_imports)]
use ezio::simple_socket;
#[allow(unused_imports)]
use ezio::custom_socket;
#[allow(unused_imports)]
use ezio::port_scanner;
#[allow(unused_imports)]
use ezio::web_server_v2;
#[allow(unused_imports)]
use ezio::dhcp_server;
#[allow(unused_imports)]
use ezio::cookbook;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    // let args: Vec<String> = env::args().collect();
    // simple_socket::run(&args);
    // custom_socket::run(&args);
    // port_scanner::run(&args);
    // web_server_v2::run(&args);
    // dhcp_server::run();
    // cookbook::algorithms::generate_random_values::random_password_custom();
    // cookbook::algorithms::sort_a_vector::sort_a_vector_of_struct();
    // cookbook::command_line::argument_parsing::run();
    // cookbook::command_line::ansi_terminal::print_bold_and_coloured_text();
    // cookbook::compression::compress_into_tarball();
    // cookbook::concurrency::explicit_threads::draw_fractal_image();
    // cookbook::concurrency::data_parallelism::parallel_gen_thumbnail();
    // cookbook::cryptography::hashing::verify_sign();
    // cookbook::cryptography::encryption::password_with_pbkdf2();
    // cookbook::data_structures::run();
    // cookbook::database::sqlite::transaction();
    cookbook::database::postgres::aggregate();
}