use super::{handshake, HandshakeError, HandshakeResult};
use bitcoin::{p2p::ServiceFlags, Network};
use std::{fmt::Debug, time::Duration};
use tokio::net::TcpStream;
use tokio::net::ToSocketAddrs;
use tracing::{info, Instrument};

/// Local node descriptor
#[derive(Debug)]
pub struct Node {
    pub(crate) services: ServiceFlags,
    pub(crate) start_height: i32,
    pub(crate) network: Network,
}

impl Node {
    pub fn new(services: u64, start_height: i32, network: Network) -> Self {
        Self {
            services: ServiceFlags::from(services),
            start_height,
            network,
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            services: ServiceFlags::NONE,
            start_height: 0,
            network: Network::Bitcoin,
        }
    }
}

impl Node {
    /// Connects to a remote node and performs a handshake returning underlying transport as result.
    /// Accepts `timeout` as maximal handshake duration.
    pub async fn connect<A: ToSocketAddrs + Debug>(
        &self,
        peer_address: A,
        timeout: Duration,
    ) -> Result<HandshakeResult<TcpStream>, HandshakeError> {
        let span = tracing::span!(tracing::Level::TRACE, "handshake", peer = ?peer_address);

        tokio::time::timeout(timeout, async {
            info!(message = format!("Local node: {:#?}", self));
            let stream = TcpStream::connect(peer_address).await?;
            let src_addr = stream.local_addr()?;
            let dst_addr = stream.peer_addr()?;
            handshake(self, &src_addr, &dst_addr, stream).await
        })
        .instrument(span)
        .await?
    }
}
