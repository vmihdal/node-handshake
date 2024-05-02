//! Bitcoin handshake implementation.
//!
//! Example:
//! ```ignore
//! use std::time::Duration;
//! use handshake::bitcoin::{Node,HandshakeResult};
//!
//! let node = Node::default();
//! let remote = "45.129.182.80:8333";
//! let timeout = Duration::from_millis(5000);
//! let HandshakeResult{ transport, messages } = node.connect(remote, timeout).await?;
//! //Use transport to communicate with remote node
//! ```

mod codec;
mod error;
mod handshake;
mod log;
mod node;
mod types;

pub use bitcoin::p2p::ServiceFlags;
pub use bitcoin::Network;
pub use codec::*;
pub use error::HandshakeError;
pub use handshake::handshake;
pub use node::Node;
pub use types::HandshakeResult;
