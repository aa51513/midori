use std::convert::{TryFrom};
use std::io::Result;
use std::net::SocketAddr;
use std::sync::Arc;

use log::debug;
use async_trait::async_trait;
use webpki::{DnsName};
use rustls::{ClientConfig, ServerConfig};
use tokio_rustls::{TlsAcceptor, TlsConnector};
// re-export
pub use tokio_rustls::client::TlsStream as ClientTLSStream;
pub use tokio_rustls::server::TlsStream as ServerTLSStream;

use super::{AsyncConnect, AsyncAccept, Transport};
use crate::utils::{self, CommonAddr};

pub struct Connector<T: AsyncConnect> {
    cc: T,
    sni: DnsName,
    // includes inner tls config
    tls: TlsConnector,
}

impl<T: AsyncConnect> Connector<T> {
    pub fn new(cc: T, sni: DnsName, tlsc: ClientConfig) -> Self {
        Self {
            cc,
            sni,
            tls: TlsConnector::from(Arc::new(tlsc)),
        }
    }
}

#[async_trait]
impl<T: AsyncConnect> AsyncConnect for Connector<T> {
    const TRANS: Transport = Transport::TLS;

    const SCHEME: &'static str = "tls";

    type IO = ClientTLSStream<T::IO>;

    #[inline]
    fn addr(&self) -> &CommonAddr { self.cc.addr() }

    fn clear_reuse(&self) {}

    async fn connect(&self) -> Result<Self::IO> {
        let stream = self.cc.connect().await?;
        debug!("tls connect ->");
        let dns_name_ref = self.sni.as_ref();
        let server_name_str  = std::str::from_utf8(dns_name_ref.as_ref()).unwrap();
        let server_name = rustls::ServerName::try_from(server_name_str).expect("invalid DNS name");
        self.tls.connect(server_name, stream).await
    }
}

pub struct Acceptor<T: AsyncAccept> {
    lis: T,
    // includes inner tls config
    tls: TlsAcceptor,
}

impl<T: AsyncAccept> Acceptor<T> {
    pub fn new(lis: T, tlsc: ServerConfig) -> Self {
        Self {
            lis,
            tls: TlsAcceptor::from(Arc::new(tlsc)),
        }
    }
}

#[async_trait]
impl<T: AsyncAccept> AsyncAccept for Acceptor<T> {
    const TRANS: Transport = Transport::TLS;

    const SCHEME: &'static str = "tls";

    type IO = ServerTLSStream<T::IO>;

    type Base = T::Base;

    #[inline]
    fn addr(&self) -> &utils::CommonAddr { self.lis.addr() }

    #[inline]
    async fn accept_base(&self) -> Result<(Self::Base, SocketAddr)> {
        self.lis.accept_base().await
    }

    async fn accept(&self, base: Self::Base) -> Result<Self::IO> {
        let stream = self.lis.accept(base).await?;
        debug!("tls accept <-");
        Ok(self.tls.accept(stream).await?)
    }
}
