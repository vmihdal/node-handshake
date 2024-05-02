use std::fmt::Debug;
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

use bitcoin::p2p::message::{NetworkMessage, RawNetworkMessage};
use bitcoin::p2p::message_network::VersionMessage;
use bitcoin::p2p::{Address, ServiceFlags};
use futures::{SinkExt, TryStreamExt};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::Framed;

use crate::bitcoin::log;

use super::codec::NetworkMessageCodec;
use super::error::HandshakeError;
use super::node::Node;
use super::types::*;

//Some handshake states for internal use
enum HandshakeStates {
    VersionSent,
    VerackSent,
}

/// Minimal version of bitcoin node( from protocol implementation )
const MIN_PEER_PROTO_VERSION: u32 = 31800;

/// Performs a handshake returning underlying transport as result.
pub async fn handshake<T: AsyncRead + AsyncWrite + Debug + Unpin>(
    src: &Node,
    src_address: &SocketAddr,
    dst_address: &SocketAddr,
    transport: T,
) -> Result<HandshakeResult<T>, HandshakeError> {
    let codec = NetworkMessageCodec {};
    let mut transport = Framed::new(transport, codec);
    let mut messages = Vec::new();

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let receiver_address = Address::new(dst_address, ServiceFlags::NONE);
    let sender_address = Address::new(src_address, src.services);
    let nonce = timestamp.as_secs();
    let timestamp = timestamp.as_secs() as i64;
    let version = VersionMessage::new(
        src.services,
        timestamp,
        receiver_address,
        sender_address,
        nonce,
        Default::default(),
        src.start_height,
    );
    let magic = src.network.magic();
    let message = RawNetworkMessage::new(magic, NetworkMessage::Version(version));
    transport.send(&message).await?;
    log::info_out_message(message.payload());
    messages.push(message);

    let mut state = HandshakeStates::VersionSent;

    while let Some(raw) = transport.try_next().await? {
        log::info_in_message(raw.payload());
        messages.push(raw.to_owned());
        let payload = raw.payload();

        match state {
            HandshakeStates::VersionSent => match payload {
                NetworkMessage::Version(peer_version) => {
                    if peer_version.version < MIN_PEER_PROTO_VERSION {
                        return Err(HandshakeError::ProtocolMistmatch);
                    }

                    if !peer_version.services.has(src.services) {
                        return Err(HandshakeError::ServicesMistmatch);
                    }

                    let message = RawNetworkMessage::new(magic, NetworkMessage::Verack);
                    transport.send(&message).await?;
                    log::info_out_message(message.payload());
                    messages.push(message);
                    state = HandshakeStates::VerackSent;
                }
                _ => break,
            },
            HandshakeStates::VerackSent => match payload {
                NetworkMessage::Verack => {
                    let result = HandshakeResult::<T> {
                        transport,
                        messages,
                    };
                    return Ok(result);
                }
                _ => break,
            },
        }
    }

    Err(HandshakeError::MessageOrder(messages))
}
