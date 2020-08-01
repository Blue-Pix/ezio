use std::env;

use pnet::datalink::{
    self,
    Channel::Ethernet,   
};
use pnet::packet::{
    Packet,
    ethernet::{
        EthernetPacket,
        EtherTypes,
    },
    ip::IpNextHeaderProtocols,
    ipv4::Ipv4Packet,
    ipv6::Ipv6Packet,
    tcp::TcpPacket,
    udp::UdpPacket,
};

use ezio::tcp_server;
use ezio::tcp_client;
use ezio::udp_server;
use ezio::udp_client;
use ezio::packets::GettableEndPoints;

#[macro_use]
extern crate log;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    // simple_socket(&args);
    custom_socket(&args);
}

fn missing_role() {
    error!("Please specify server or client on the 2nd argument.");
    std::process::exit(1);
}

fn simple_socket(args: &Vec<String>) {
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

fn custom_socket(args: &Vec<String>) {
    if args.len() != 2 {
        error!("Please specify target interface name");
        std::process::exit(1);
    }
    let interface_name = &args[1];
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter().find(|iface| {
        println!("{}", iface.name); 
        iface.name == *interface_name
    }).expect("Failed to get interface");

    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Failed to create datalink channel {}", e),
    };

    loop {
        match rx.next() {
            Ok(frame) => {
                let frame = EthernetPacket::new(frame).unwrap();
                match frame.get_ethertype() {
                    EtherTypes::Ipv4 => {
                        ipv4_handler(&frame);
                    },
                    EtherTypes::Ipv6 => {
                        ipv6_handler(&frame);
                    },
                    _ => {
                        info!("Not an IPv4 nor IPv6 packet.");
                    }
                }
            },
            Err(e) => {
                error!("Failed to read: {}", e);
            }
        }
    }
}

fn ipv4_handler(ethernet: &EthernetPacket) {
    if let Some(packet) = Ipv4Packet::new(ethernet.payload()) {
        match packet.get_next_level_protocol() {
            IpNextHeaderProtocols::Tcp => {
                tcp_handler(&packet);
            },
            IpNextHeaderProtocols::Udp => {
                udp_handler(&packet);
            },
            _ => {
                info!("Not a TCP nor UDP packet.");
            }
        }
    }
}

fn ipv6_handler(ethernet: &EthernetPacket) {
    if let Some(packet) = Ipv6Packet::new(ethernet.payload()) {
        match packet.get_next_header() {
            IpNextHeaderProtocols::Tcp => {
                tcp_handler(&packet);
            },
            IpNextHeaderProtocols::Udp => {
                udp_handler(&packet);
            },
            _ => {
                info!("Not a TCP nor UDP packet.");
            }
        };
    }
}

fn tcp_handler(packet: &dyn GettableEndPoints) {
    let tcp = TcpPacket::new(packet.get_payload());
    if let Some(tcp) = tcp {
        print_packet_info(packet, &tcp, "TCP");
    } 
}

fn udp_handler(packet: &dyn GettableEndPoints) {
    let udp = UdpPacket::new(packet.get_payload());
    if let Some(udp) = udp {
        print_packet_info(packet, &udp, "UDP");
    }
}

fn print_packet_info(l3: &dyn GettableEndPoints, l4: &dyn GettableEndPoints, proto: &str) {
    const WIDTH: usize = 40;
    println!("Captured a {} packet from {}|{} to {}|{}\n", 
        proto, 
        l3.source(), 
        l4.source(), 
        l3.destination(), 
        l4.destination(),
    );
    let payload = l4.get_payload();
    let len = payload.len();

    for i in 0..len {
        print!("{:<02X}", payload[i]);
        if i % WIDTH == WIDTH - 1 || i == len - 1 {
            for _j in 0..WIDTH - 1 - (i % (WIDTH)) {
                print!(" ");
            }
            print!("| ");
            for j in i - i % WIDTH..=i {
                if payload[j].is_ascii_alphabetic() {
                    print!("{}", payload[j] as char);
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
    println!("{}", "=".repeat(WIDTH * 3));
    println!();
}