const LOCAL_IP_PREFIX: &str = "192.168.";

pub fn list() -> Vec<(String, std::net::Ipv4Addr)> {
  let network_interfaces = local_ip_address::list_afinet_netifas().unwrap();

  network_interfaces.into_iter().filter_map(|(name, ip)| {
    if let std::net::IpAddr::V4(ipv4) = ip {
      if ip.to_string().starts_with(LOCAL_IP_PREFIX) {
        Some((name, ipv4))
      } else {
        None
      }
    } else {
      None
    }
  }).collect()
}