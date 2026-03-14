use mdns_sd::{ServiceDaemon, ServiceInfo};

const SERVICE_TYPE: &str = "_lumehub._tcp.local.";

const ROUTE_PROBE_ADDR: &str = "8.8.8.8:80";

pub struct Config {
    pub name: String,
    pub port: u16,
}

pub fn advertise(config: &Config) -> Result<ServiceDaemon, mdns_sd::Error> {
    let mdns = ServiceDaemon::new()?;

    let host_name = format!("{}.local.", hostname());
    let ip = lan_ip().to_string();

    let service = ServiceInfo::new(
        SERVICE_TYPE,
        &config.name,
        &host_name,
        ip.as_str(),
        config.port,
        None,
    )?;

    mdns.register(service)?;

    println!(
        "mDNS: advertising {} at {}:{} ({})",
        config.name, ip, config.port, SERVICE_TYPE
    );

    Ok(mdns)
}

pub fn lan_ip() -> std::net::Ipv4Addr {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").expect("UDP socket bind");
    let _ = socket.connect(ROUTE_PROBE_ADDR);
    match socket.local_addr().ok().map(|a| a.ip()) {
        Some(std::net::IpAddr::V4(ip)) if !ip.is_unspecified() => ip,
        _ => std::net::Ipv4Addr::LOCALHOST,
    }
}

pub fn hostname() -> String {
    std::fs::read_to_string("/etc/hostname")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "lumehub".to_string())
}
