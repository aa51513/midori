use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use async_trait::async_trait;
use webpki::DNSName;
use rustls::{ClientConfig, ServerConfig};
use tokio_rustls::{TlsAcceptor, TlsConnector};
// re-export
pub use tokio_rustls::client::TlsStream as ClientTLSStream;
pub use tokio_rustls::server::TlsStream as ServerTLSStream;

use super::{AsyncConnect, AsyncAccept};
use super::plain::PlainStream;
use crate::utils;

#[derive(Clone)]
pub struct Connector<T: AsyncConnect> {
    cc: T,
    sni: DNSName,
    // includes inner tls config
    tls: TlsConnector,
}

impl<T: AsyncConnect> Connector<T> {
    pub fn new(cc: T, sni: DNSName, tlsc: ClientConfig) -> Self {
        Self {
            cc,
            sni,
            tls: TlsConnector::from(Arc::new(tlsc)),
        }
    }
}

#[async_trait]
impl<T: AsyncConnect> AsyncConnect for Connector<T> {
    type IO = ClientTLSStream<T::IO>;
    async fn connect(&self) -> io::Result<Self::IO> {
        let stream = self.cc.connect().await?;
        self.tls.connect(self.sni.as_ref(), stream).await
    }
}

#[derive(Clone)]
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
    type IO = ServerTLSStream<T::IO>;

    fn addr(&self) -> &utils::CommonAddr { self.lis.addr() }

    async fn accept(
        &self,
        res: (PlainStream, SocketAddr),
    ) -> io::Result<(Self::IO, SocketAddr)> {
        let (stream, addr) = self.lis.accept(res).await?;
        Ok((self.tls.accept(stream).await?, addr))
    }
}