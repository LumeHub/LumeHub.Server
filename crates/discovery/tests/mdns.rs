use discovery::{advertise, hostname, lan_ip};

#[test]
fn hostname_is_non_empty_and_single_line() {
    let h = hostname();
    assert!(!h.is_empty());
    assert!(!h.contains('\n'));
}

#[test]
fn lan_ip_is_valid_ipv4() {
    let ip = lan_ip();
    assert!(!ip.is_unspecified(), "expected a routable IP, got 0.0.0.0");
}

#[test]
fn service_is_discoverable() {
    use std::net::{Ipv4Addr, UdpSocket};
    use std::time::{Duration, Instant};

    use mdns_sd::{ServiceDaemon, ServiceEvent};

    // Probe multicast availability before committing to the test.
    let probe = UdpSocket::bind("0.0.0.0:0").unwrap();
    if probe
        .join_multicast_v4(&"224.0.0.251".parse().unwrap(), &Ipv4Addr::UNSPECIFIED)
        .is_err()
    {
        eprintln!("service_is_discoverable: multicast unavailable, skipping");
        return;
    }

    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();

    let _server = advertise(&discovery::Config {
        name: "LumeHub-test".to_string(),
        port,
    })
    .expect("advertise failed");

    let browser = ServiceDaemon::new().expect("browse daemon");
    let rx = browser
        .browse("_lumehub._tcp.local.")
        .expect("browse failed");

    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        match rx.recv_timeout(Duration::from_millis(200)) {
            Ok(ServiceEvent::ServiceResolved(info)) if info.get_port() == port => return,
            Ok(_) | Err(_) => continue,
        }
    }

    panic!("LumeHub._lumehub._tcp.local. not discovered within 5s");
}
