pub fn list() -> Vec<(String, std::net::Ipv4Addr)> {
  let network_interfaces = local_ip_address::list_afinet_netifas().unwrap();

  network_interfaces.into_iter().filter_map(|(name, ip)| {
    if let std::net::IpAddr::V4(ipv4) = ip {
      if name.to_lowercase().contains("wi-fi") {
        Some((name, ipv4))
      } else {
        None
      }
    } else {
      None
    }
  }).collect()
}