use bitcoin::p2p::message::{NetworkMessage, RawNetworkMessage};
use futures::SinkExt;
use tokio_util::codec::Framed;

use crate::bitcoin::{handshake, HandshakeError, NetworkMessageCodec, Node};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

const MAX_HANDSHAKE_DURATION: u64 = 30;

#[tokio::test]
async fn connect_two_local_nodes() -> Result<(), Box<dyn std::error::Error>> {
    let node = Node::default();
    let src = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
    let dst = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255)), 8333);
    let (local, remote) = tokio::io::duplex(bitcoin::p2p::message::MAX_MSG_SIZE);
    let local = handshake(&node, &src, &dst, local);
    let remote = handshake(&node, &dst, &src, remote);
    let local = tokio::time::timeout(Duration::from_secs(MAX_HANDSHAKE_DURATION), local);
    let remote = tokio::time::timeout(Duration::from_secs(MAX_HANDSHAKE_DURATION), remote);
    let result = tokio::try_join!(local, remote);
    assert!(result.is_ok());
    Ok(())
}

#[tokio::test]
async fn wrong_message_order() -> Result<(), Box<dyn std::error::Error>> {
    let node = Node::default();
    let src = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
    let dst = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255)), 8333);
    let (local, remote) = tokio::io::duplex(bitcoin::p2p::message::MAX_MSG_SIZE);
    let mut remote = Framed::new(remote, NetworkMessageCodec {});
    let local = handshake(&node, &src, &dst, local);
    let raw = RawNetworkMessage::new(node.network.magic(), NetworkMessage::Verack);
    remote.send(&raw).await?;
    let result = tokio::time::timeout(Duration::from_secs(MAX_HANDSHAKE_DURATION), local).await?;
    assert!(matches!(result, Err(HandshakeError::MessageOrder(_))));
    Ok(())
}
