use bitcoin::p2p::message::RawNetworkMessage;
use std::time::SystemTimeError;
use thiserror::Error as ThisError;
use tokio::io::Error as TokioIoError;
use tokio::time::error::Elapsed as TimeoutError;

/// Node handshake error variants
#[derive(Debug, ThisError)]
pub enum HandshakeError {
    #[error("Handshake timeout: {0}")]
    Timeout(#[from] TimeoutError),
    #[error("Tokio io error: {0}")]
    Io(#[from] TokioIoError),
    #[error("System time generation error: {0}")]
    SystemTime(#[from] SystemTimeError),
    #[error("Wrong or unsupported protocol version")]
    ProtocolMistmatch,
    #[error("Peer node does not provide services expected by local node")]
    ServicesMistmatch,
    #[error("Wrong message order")]
    MessageOrder(Vec<RawNetworkMessage>),
    #[error("Network message encoding/decoding error: {0}")]
    Codec(#[from] NetworkMessageCodecError),
}

/// Network message en/decoding error variants
#[derive(Debug, ThisError)]
pub enum NetworkMessageCodecError {
    #[error("Tokio io error: {0}")]
    Io(#[from] TokioIoError),
    #[error("Decode error")]
    Decode(#[from] bitcoin::consensus::encode::Error),
}
