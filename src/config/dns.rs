use serde::{Serialize, Deserialize};
use trust_dns_resolver::config::{LookupIpStrategy,NameServerConfig};
use trust_dns_resolver::config::Protocol;

use std::net::SocketAddr;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DnsMode {
    /// Only query for A (Ipv4) records
    Ipv4Only,
    /// Only query for AAAA (Ipv6) records
    Ipv6Only,
    /// Query for A and AAAA in parallel
    Ipv4AndIpv6,
    /// Query for Ipv4 if that fails, query for Ipv6 (default)
    Ipv4ThenIpv6,
    /// Query for Ipv6 if that fails, query for Ipv4
    Ipv6ThenIpv4,
}

impl Default for DnsMode {
    fn default() -> Self { Self::Ipv4ThenIpv6 }
}

impl From<DnsMode> for LookupIpStrategy {
    fn from(mode: DnsMode) -> Self {
        match mode {
            DnsMode::Ipv4Only => LookupIpStrategy::Ipv4Only,
            DnsMode::Ipv6Only => LookupIpStrategy::Ipv6Only,
            DnsMode::Ipv4AndIpv6 => LookupIpStrategy::Ipv4AndIpv6,
            DnsMode::Ipv4ThenIpv6 => LookupIpStrategy::Ipv4thenIpv6,
            DnsMode::Ipv6ThenIpv4 => LookupIpStrategy::Ipv6thenIpv4,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DnsServerConfig {
    pub addr: String,

    #[serde(default)]
    pub protocol: DnsProtocol,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DnsProtocol {
    Udp,
    Tcp,
}

impl Default for DnsProtocol {
    fn default() -> Self { Self::Udp }
}

impl From<DnsProtocol> for Protocol {
    fn from(mode: DnsProtocol) -> Self {
        match mode {
            DnsProtocol::Udp => Protocol::Udp,
            DnsProtocol::Tcp => Protocol::Tcp,
        }
    }
}

impl From<DnsServerConfig> for NameServerConfig {
    fn from(x: DnsServerConfig) -> NameServerConfig {
        println!("x.addr is {}",x.addr);
        let socket_addr: SocketAddr = x.addr
                .parse()
                .expect("Unable to parse socket address");
        NameServerConfig {
            socket_addr: socket_addr,
            protocol: Protocol::from(x.protocol),
            tls_dns_name: None,
            trust_nx_responses: true,
        }
    }
}