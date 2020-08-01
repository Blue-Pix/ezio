use pnet::packet::{
  Packet,
  ipv4::Ipv4Packet,
  ipv6::Ipv6Packet,
  tcp::TcpPacket,
  udp::UdpPacket,
};

pub trait GettableEndPoints {
  fn source(&self) -> String;
  fn destination(&self) -> String;
  fn get_payload(&self) -> &[u8];
}

impl<'a> GettableEndPoints for Ipv4Packet<'a> {
  fn source(&self) -> String {
    self.get_source().to_string()
  }

  fn destination(&self) -> String {
    self.get_destination().to_string()
  }

  fn get_payload(&self) -> &[u8] {
    self.payload()
  }
}

impl<'a> GettableEndPoints for Ipv6Packet<'a> {
  fn source(&self) -> String {
    self.get_source().to_string()
  }

  fn destination(&self) -> String {
    self.get_destination().to_string()
  }

  fn get_payload(&self) -> &[u8] {
    self.payload()
  }
}

impl<'a> GettableEndPoints for TcpPacket<'a> {
  fn source(&self) -> String {
    self.get_source().to_string()
  }

  fn destination(&self) -> String {
    self.get_destination().to_string()
  }

  fn get_payload(&self) -> &[u8] {
    self.payload()
  }
}

impl<'a> GettableEndPoints for UdpPacket<'a> {
  fn source(&self) -> String {
    self.get_source().to_string()
  }

  fn destination(&self) -> String {
    self.get_destination().to_string()
  }

  fn get_payload(&self) -> &[u8] {
    self.payload()
  }
}