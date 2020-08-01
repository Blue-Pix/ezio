mod tcp_server;
mod tcp_client;
mod udp_server;
mod udp_client;

pub fn run(args: &Vec<String>) {
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
              udp_client::communicate(address).unwrap_or_else(|e| {
                  error!("{}", e);
              })
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