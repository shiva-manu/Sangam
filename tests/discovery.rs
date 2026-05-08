//! Smoke tests for the discovery layer.
//!
//! We can't easily drive a real mDNS daemon in CI, so these tests focus on
//! the pure-logic helpers exposed from the module.

use std::net::IpAddr;

use Sangam::discovery::mdns::{SERVICE_TYPE, pick_peer_address};

#[test]
fn service_type_uses_correct_mdns_format() {
    // mdns-sd refuses to register a service whose type doesn't end in
    // either `._tcp.local.` or `._udp.local.`. This test guards against
    // an accidental regression of the original
    // `_sangam.udp.local.` (no underscore) bug.
    assert!(
        SERVICE_TYPE.ends_with("._tcp.local.") || SERVICE_TYPE.ends_with("._udp.local."),
        "SERVICE_TYPE `{}` must end with `._tcp.local.` or `._udp.local.`",
        SERVICE_TYPE
    );
    assert!(
        SERVICE_TYPE.starts_with('_'),
        "SERVICE_TYPE `{}` must start with an underscore",
        SERVICE_TYPE
    );
}

fn ip(s: &str) -> IpAddr {
    s.parse().unwrap()
}

#[test]
fn pick_peer_address_prefers_ipv4_lan_over_loopback() {
    let addrs = [ip("127.0.0.1"), ip("192.168.1.42")];
    let chosen = pick_peer_address(addrs.iter()).copied();
    assert_eq!(chosen, Some(ip("192.168.1.42")));
}

#[test]
fn pick_peer_address_skips_link_local() {
    let addrs = [ip("169.254.10.5"), ip("10.0.0.7")];
    let chosen = pick_peer_address(addrs.iter()).copied();
    assert_eq!(chosen, Some(ip("10.0.0.7")));
}

#[test]
fn pick_peer_address_falls_back_to_ipv6_when_no_ipv4() {
    let addrs = [ip("::1"), ip("2001:db8::1")];
    let chosen = pick_peer_address(addrs.iter()).copied();
    assert_eq!(
        chosen,
        Some(ip("2001:db8::1")),
        "should pick non-loopback IPv6 over IPv6 loopback"
    );
}

#[test]
fn pick_peer_address_returns_none_for_empty_set() {
    let addrs: Vec<IpAddr> = vec![];
    assert_eq!(pick_peer_address(addrs.iter()).copied(), None);
}

#[test]
fn pick_peer_address_falls_back_to_loopback_when_only_option() {
    // Last-resort: at least try *something* rather than dropping the peer.
    let addrs = [ip("127.0.0.1")];
    let chosen = pick_peer_address(addrs.iter()).copied();
    assert_eq!(chosen, Some(ip("127.0.0.1")));
}
