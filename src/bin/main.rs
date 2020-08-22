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
    // cookbook::database::postgres::aggregate();
    // cookbook::date_and_time::duration_and_calculation::local_timezone();
    // cookbook::date_and_time::parsing_and_displaying::parse();
    // cookbook::development_tools::debugging::custom_location();
    // let _ = cookbook::development_tools::versioning::external_command_version();
    // let _ = cookbook::development_tools::build_time_tooling::custom_define();
    // let _ = cookbook::encoding::character_sets::_base64();
    // let _ = cookbook::encoding::csv_processing::transform_csv_column();
    // let _ = cookbook::encoding::structured_data::little_endian();
    // match cookbook::error_handling::read_uptime() {
    //     Ok(uptime) => println!("uptime: {} seconds", uptime),
    //     Err(err) => eprintln!("error: {}", err)
    // }
    // cookbook::error_handling::obtain_backtrace();
    // let _ = cookbook::file_system::read_write::access_file_randomly();
    // let _ = cookbook::file_system::directory_traversal::glob_with_option();
    // let _ = cookbook::hardware_support::count_cpu_cores();
    // let _ = cookbook::memory_management::lazy_constant();
    // if let Err(e) = cookbook::networking::assign_unused_port() {
    //     eprintln!("{}", e);
    // }
    // let _ = cookbook::operating_system::read_env();
    cookbook::science::calc_big_integer();
}