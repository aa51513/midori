use std::borrow::BorrowMut;
use std::io::{Result, Error, ErrorKind};
use std::net::IpAddr;
use std::sync::{Mutex, Once};

use futures::executor;
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts,NameServerConfig,NameServerConfigGroup};
use lazy_static::lazy_static;

use crate::config::{DnsServerConfig};

static mut NAME_SERVER_CONFIGS:Vec<NameServerConfig> = vec![];

static mut STD_ONCE_COUNTER: Option<Mutex<ResolverOpts>> = None;
static INIT: Once = Once::new();
fn global_string<'a>() -> &'a Mutex<ResolverOpts> {
    INIT.call_once(|| {
        // Since this access is inside a call_once, before any other accesses, it is safe
        unsafe {
            *STD_ONCE_COUNTER.borrow_mut() = Some(Mutex::new(ResolverOpts::default()));
        }
    });
    // As long as this function is the only place with access to the static variable,
    // giving out a read-only borrow here is safe because it is guaranteed no more mutable
    // references will exist at this point or in the future.
    unsafe { STD_ONCE_COUNTER.as_ref().unwrap() }
}

lazy_static! {

    static ref DNS: TokioAsyncResolver = {
        let config: ResolverConfig = ResolverConfig::from_parts(None,vec![],unsafe { NameServerConfigGroup::from(NAME_SERVER_CONFIGS.clone())});
        let resolver_opts = *global_string().lock().unwrap();
        TokioAsyncResolver::tokio(config,resolver_opts).unwrap()
    };
}

pub fn init_resolver(resolver_opts: ResolverOpts,dns_servers: Vec<DnsServerConfig>) {
    *global_string().lock().unwrap() = resolver_opts;

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
