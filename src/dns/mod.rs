use std::io::{Result, Error, ErrorKind};
use std::net::IpAddr;

use futures::executor;
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts, LookupIpStrategy,NameServerConfig,NameServerConfigGroup};
use lazy_static::lazy_static;

use crate::config::{DnsServerConfig};

static mut RESOLVE_STRATEGY: LookupIpStrategy = LookupIpStrategy::Ipv4thenIpv6;
static mut NAME_SERVER_CONFIGS:Vec<NameServerConfig> = vec![];

lazy_static! {
    let config: ResolverConfig = ResolverConfig::from_parts(None,vec![],unsafe { NameServerConfigGroup::from(NAME_SERVER_CONFIGS.clone())});
    let options: ResolverOpts = ResolverOpts {
            ip_strategy: unsafe { RESOLVE_STRATEGY },
            ..Default::default()
        };
    static ref DNS: TokioAsyncResolver = TokioAsyncResolver::tokio(config,options).unwrap();
}

pub fn init_resolver(strategy: LookupIpStrategy,dns_servers: Vec<DnsServerConfig>) {

    unsafe { RESOLVE_STRATEGY = strategy };

    let name_server_configs : Vec<NameServerConfig> = dns_servers.into_iter().map(NameServerConfig::from).collect();
    for name_server_config in name_server_configs {
        unsafe { NAME_SERVER_CONFIGS.push(name_server_config)};
    }

    lazy_static::initialize(&DNS);
}

pub fn resolve_sync(addr: &str) -> Result<IpAddr> {
    executor::block_on(resolve_async(addr))
}

pub async fn resolve_async(addr: &str) -> Result<IpAddr> {
    let res = DNS
        .lookup_ip(addr)
        .await
        .map_err(|e| Error::new(ErrorKind::Other, e))?
        .into_iter()
        .next()
        .unwrap();
    Ok(res)
}
