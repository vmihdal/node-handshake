use std::io::Cursor;

use bitcoin::consensus::Decodable;
use bitcoin::p2p::message::{RawNetworkMessage, MAX_MSG_SIZE};
use tokio_util::bytes::{Buf, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use super::error::NetworkMessageCodecError;

/// Network message header size in bytes
pub const MESSAGE_HEADER_SIZE: usize = 24;
/// Offset in bytes to payload size in a header
const PAYLOAD_SIZE_OFFSET: u64 = 16;

/// Bitcoin network message encoder and decoder
pub struct NetworkMessageCodec {}

impl Decoder for NetworkMessageCodec {
    type Item = RawNetworkMessage;
    type Error = NetworkMessageCodecError;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < MESSAGE_HEADER_SIZE {
            return Ok(None);
        }

        let mut cursor = Cursor::new(&src);

        cursor.set_position(PAYLOAD_SIZE_OFFSET);

        let payload_size = u32::consensus_decode_from_finite_reader(&mut cursor)? as usize;

        if src.len() < MESSAGE_HEADER_SIZE + payload_size {
            return Ok(None);
        }

        if src.len() > MAX_MSG_SIZE {
            let kind = tokio::io::ErrorKind::InvalidData;
            return Err(tokio::io::Error::from(kind).into());
        }

        cursor.set_position(0);

        match RawNetworkMessage::consensus_decode_from_finite_reader(&mut cursor) {
            Ok(decoded) => {
                src.advance(cursor.position() as usize);
                Ok(Some(decoded))
            }
            Err(error) => Err(error)?,
        }
    }
}

impl Encoder<&RawNetworkMessage> for NetworkMessageCodec {
    type Error = NetworkMessageCodecError;

    fn encode(&mut self, src: &RawNetworkMessage, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let data = bitcoin::consensus::serialize(src);
        dst.extend(data);
        Ok(())
    }
}
