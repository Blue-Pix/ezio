use std::env;
use ezio::tcp_server;
use ezio::tcp_client;
use ezio::udp_server;

#[macro_use]
extern crate log;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        error!("Please specify [tcp|udp] [server|client] [addr:port].");
        std::process::exit(1);
    }
    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address = &args[3];
    match protocol {
        "tcp" => match role {
            "server" => {
                tcp_server::serve(address).unwrap_or_else(|e| {
                    error!("{}", e);
                });
            },
            "client" => {
                tcp_client::connect(address).unwrap_or_else(|e| {
                    error!("{}", e);
                });
            },
            _ => {
                missing_role();
            }
        },
        "udp" => match role {
            "server" => {
                udp_server::serve(address).unwrap_or_else(|e| {
                    error!("{}", e);
                });
            },
            "client" => {
                //
            },
            _ => {
                missing_role();
            }
        },
        _ => {
            error!("Please specify tcp or udp on the 1st argument.");
            std::process::exit(1);
        }
    }
}

fn missing_role() {
    error!("Please specify server or client on the 2nd argument.");
    std::process::exit(1);
}