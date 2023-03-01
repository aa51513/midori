use std::io::{Result, Error, ErrorKind};
use std::net::SocketAddr;
use std::sync::Arc;

use log::{warn, info, debug};
use async_trait::async_trait;

use quinn::{Connection, Endpoint};

use super::QuicStream;
use crate::utils::{CommonAddr};
use crate::transport::{AsyncConnect, AsyncAccept, Transport};

pub struct Acceptor<C> {
    cc: Arc<C>,
    lis: Endpoint,
    addr: CommonAddr,
}

impl<C> Acceptor<C> {
    pub fn new(cc: Arc<C>, lis: Endpoint, addr: CommonAddr) -> Self {
        Acceptor { cc, lis, addr }
    }
}

// Single Connection
#[async_trait]
impl AsyncAccept for Acceptor<()> {
    const TRANS: Transport = Transport::QUIC;

    const SCHEME: &'static str = "quic";

    type IO = QuicStream;

    type Base = QuicStream;

    fn addr(&self) -> &CommonAddr { &self.addr }

    async fn accept_base(&self) -> Result<(Self::Base, SocketAddr)> {
        // new connection
        let connecting = (&self).lis.accept().await.ok_or_else(|| {
            Error::new(ErrorKind::ConnectionAborted, "connection abort")
        })?;

        // early data
        let new_conn = match connecting.into_0rtt() {
            Ok((new_conn, _)) => new_conn,
            Err(connecting) => connecting.await?,
        };
        debug!("quic accept[new] <- {}", &new_conn.remote_address());

        let (send, recv) = new_conn.accept_bi().await.expect("no more stream");

        Ok((QuicStream::new(send, recv), new_conn.remote_address()))
    }

    async fn accept(&self, base: Self::Base) -> Result<Self::IO> { Ok(base) }
}

// Mux
#[async_trait]
impl<C> AsyncAccept for Acceptor<C>
where
    C: AsyncConnect + 'static,
{
    const TRANS: Transport = Transport::QUIC;

    const SCHEME: &'static str = "quic";

    type IO = QuicStream;

    type Base = QuicStream;

    fn addr(&self) -> &CommonAddr { &self.addr }

    async fn accept_base(&self) -> Result<(Self::Base, SocketAddr)> {
        // new connection
        let connecting = (&self).lis.accept().await.expect("connection abort");

        // early data
        let new_conn = match connecting.into_0rtt() {
            Ok((new_conn, _)) => new_conn,
            Err(connecting) => connecting.await?,
        };

        let remote_addr = new_conn.remote_address();
        debug!("quic accept[new] <- {}", &(remote_addr.clone()));

        let (send, recv) = new_conn.accept_bi().await.expect("no more stream");

        tokio::spawn(handle_mux_conn(self.cc.clone(), new_conn));
        Ok((QuicStream::new(send, recv), remote_addr))
    }

    async fn accept(&self, base: Self::Base) -> Result<Self::IO> { Ok(base) }
}

async fn handle_mux_conn<C>(cc: Arc<C>, connection: Connection)
where
    C: AsyncConnect + 'static,
{
    use crate::io::bidi_copy_with_stream;
    loop {
        match connection.accept_bi().await {
            Err(_) => {
                warn!("no more quic-mux stream");
            }
            Ok((send, recv)) =>{
                    info!(
                        "new quic stream[reuse] <-> {}[{}]",
                        cc.addr(),
                        C::SCHEME
                    );
                    tokio::spawn(bidi_copy_with_stream(
                        cc.clone(),
                        QuicStream::new(send, recv),
                    ));
                }
        }
    }
    /*
    while let Some(Ok((send, recv))) = bi_streams.next().await {
        tokio::spawn(bidi_copy_with_stream(
            cc.clone(),
            QuicStream::new(send, recv),
        ));
    }
    */
}

// Raw Acceptor, used to setup the Quic Acceptor above
pub struct RawAcceptor {
    lis: Endpoint,
    addr: CommonAddr,
}

impl RawAcceptor {
    pub fn new(lis: Endpoint, addr: CommonAddr) -> Self {
        RawAcceptor { lis, addr }
    }
    pub fn set_connector<C>(self, cc: Arc<C>) -> Acceptor<C> {
        Acceptor::new(cc, self.lis, self.addr)
    }
}

#[async_trait]
impl AsyncAccept for RawAcceptor {
    const TRANS: Transport = Transport::QUIC;

    const SCHEME: &'static str = "quic";

    type IO = QuicStream;

    type Base = QuicStream;

    fn addr(&self) -> &CommonAddr { &self.addr }

    async fn accept_base(&self) -> Result<(Self::Base, SocketAddr)> {
        // new connection
        let connecting = (&self).lis.accept().await.expect("connection abort");

        // early data
        let new_conn = match connecting.into_0rtt() {
            Ok((new_conn, _)) => new_conn,
            Err(connecting) => connecting.await?,
        };

        let (send, recv) = new_conn.accept_bi().await.expect("no more stream");

        Ok((QuicStream::new(send, recv), new_conn.remote_address()))
    }

    async fn accept(&self, base: Self::Base) -> Result<Self::IO> { Ok(base) }
}
