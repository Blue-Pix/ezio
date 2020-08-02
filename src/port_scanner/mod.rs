
use std::collections::HashMap;
use std::fs;
use std::net;
use std::thread;
use std::time::Duration;
use pnet::{
  packet::{
    ip::IpNextHeaderProtocols,
    tcp::{
      self, 
      MutableTcpPacket, 
      TcpFlags
    },
  },
  transport::{
    self, 
    TransportChannelType, 
    TransportProtocol,
    TransportReceiver,
    TransportSender,
  }
};
use rayon;

const TCP_SIZE: usize = 20;

#[derive(Debug)]
struct PacketInfo {
  my_ipaddr: net::Ipv4Addr,
  target_ipaddr: net::Ipv4Addr,
  my_port: u16,
  maximum_port: u16,
  scan_type: ScanType,
}

#[derive(Copy, Clone, Debug)]
enum ScanType {
  Syn = TcpFlags::SYN as isize,
  Fin = TcpFlags::FIN as isize,
  Xmas = (TcpFlags::FIN | TcpFlags::URG | TcpFlags::PSH) as isize,
  Null = 0, 
}

pub fn run(args: &Vec<String>) {
  if args.len() != 3 {
    error!("Bad number of arguments. [ipaddr] [scantype]");
    std::process::exit(1);
  }

  let packet_info = {
    let contents = fs::read_to_string(".env").expect("Failed to read the env file.");
    let lines: Vec<_> = contents.split('\n').collect();
    let mut map = HashMap::new();
    for line in lines {
      let elm: Vec<_> = line.split('=').map(str::trim).collect();
      if elm.len() == 2 {
        map.insert(elm[0], elm[1]);
      }
    }
    PacketInfo {
      my_ipaddr: map["MY_IPADDR"].parse().expect("invalid ipaddr"),
      target_ipaddr: args[1].parse().expect("invalid target ipaddr"),
      my_port: map["MY_PORT"].parse().expect("invalid port number"),
      maximum_port: map["MAXIMUM_PORT_NUM"].parse().expect("invalid maximum port num"),
      scan_type: match args[2].as_str() {
        "sS" => ScanType::Syn,
        "sF" => ScanType::Fin,
        "sX" => ScanType::Xmas,
        "sN" => ScanType::Null,
        _ => {
          error!("Undefined scan method, only accept [sS|sF|sX|sN],");
          std::process::exit(1);
        }
      },
    }
  };
  println!("{:#?}", packet_info);

  let (mut ts, mut tr) = transport::transport_channel(
    1024,
    TransportChannelType::Layer4(TransportProtocol::Ipv4(IpNextHeaderProtocols::Tcp))
  ).expect("Failed to open channel.");

  let (_, result) = rayon::join(
    || send_packet(&mut ts, &packet_info),
    || receive_packets(&mut tr, &packet_info),
  );
  match result {
    Ok(_) => {},
    Err(e) => {
      println!("Failed to join threads: {}", e);
    }
  }
}

fn build_packet(packet_info: &PacketInfo) -> [u8; TCP_SIZE] {
  let mut tcp_buffer = [0u8; TCP_SIZE];
  let mut tcp_header = MutableTcpPacket::new(&mut tcp_buffer[..]).unwrap();
  tcp_header.set_source(packet_info.my_port);
  tcp_header.set_data_offset(5);
  tcp_header.set_flags(packet_info.scan_type as u16);
  let checksum = tcp::ipv4_checksum(
    &tcp_header.to_immutable(),
    &packet_info.my_ipaddr,
    &packet_info.target_ipaddr,
  );
  tcp_header.set_checksum(checksum);
  tcp_buffer
}

fn send_packet(ts: &mut TransportSender, packet_info: &PacketInfo) {
  let mut packet = build_packet(packet_info);
  for i in 1..packet_info.maximum_port+1 {
    println!("{}", i);
    let mut tcp_header = MutableTcpPacket::new(&mut packet).unwrap();
    register_destination_port(i, &mut tcp_header, packet_info);
    thread::sleep(Duration::from_millis(5));
    ts.send_to(tcp_header, net::IpAddr::V4(packet_info.target_ipaddr)).expect(&format!("Failed to send packet to port {}", i));
  }
}

fn register_destination_port(target: u16, tcp_header: &mut MutableTcpPacket, packet_info: &PacketInfo) {
  tcp_header.set_destination(target);
  let checksum = tcp::ipv4_checksum(
    &tcp_header.to_immutable(),
    &packet_info.my_ipaddr,
    &packet_info.target_ipaddr,
  );
  tcp_header.set_checksum(checksum);
}

fn receive_packets(tr: &mut TransportReceiver, packet_info: &PacketInfo) -> Result<(), failure::Error> {
  let mut reply_ports = Vec::new();
  let mut packet_iter = transport::tcp_packet_iter(tr);
  loop {
    let tcp_packet = match packet_iter.next() {
      Ok((tcp_packet, _)) => {
        if tcp_packet.get_destination() != packet_info.my_port {
          continue;
        }
        tcp_packet
      },
      Err(_) => continue,
    };

    let target_port = tcp_packet.get_source();
    match packet_info.scan_type {
      ScanType::Syn => {
        if tcp_packet.get_flags() == TcpFlags::SYN | TcpFlags::ACK {
          println!("port {} is open", target_port);
        }
      },
      ScanType::Fin | ScanType::Xmas | ScanType::Null => {
        reply_ports.push(target_port);
      }
    }

    if target_port != packet_info.maximum_port {
      continue;
    }

    match packet_info.scan_type {
      ScanType::Fin | ScanType::Xmas | ScanType::Null => {
        for i in 1..=packet_info.maximum_port {
          if reply_ports.iter().find(|&&x| x == i).is_none() {
            println!("port {} is open", i);
          }
        }
      },
      _ => {},
    }
  }
}