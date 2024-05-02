use bitcoin::p2p::message::RawNetworkMessage;
use tokio_util::codec::Framed;

use super::codec::NetworkMessageCodec;

/// Handshake result descriptor.
pub struct HandshakeResult<T> {
    pub transport: Framed<T, NetworkMessageCodec>,
    pub messages: Vec<RawNetworkMessage>,
}
