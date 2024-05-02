use bitcoin::p2p::{
    message::{NetworkMessage, RawNetworkMessage},
    Magic,
};
use tokio_util::{
    bytes::BytesMut,
    codec::{Decoder, Encoder},
};

use crate::bitcoin::{NetworkMessageCodec, MESSAGE_HEADER_SIZE};

#[test]
fn codec() {
    //Empty
    let mut codec = NetworkMessageCodec {};
    let mut bytes = BytesMut::new();
    let result = codec.decode(&mut bytes).unwrap();
    assert!(result.is_none());

    //No enough data
    let mut bytes = BytesMut::new();
    let message = RawNetworkMessage::new(Magic::BITCOIN, NetworkMessage::Verack);
    assert!(codec.encode(&message, &mut bytes).is_ok());
    let _ = bytes.split_to(MESSAGE_HEADER_SIZE);
    let decoded = codec.decode(&mut bytes);
    assert!(decoded.unwrap().is_none());

    //To big message
    let v = vec![0u8; bitcoin::p2p::message::MAX_MSG_SIZE + 1];
    let mut bytes = BytesMut::from(&v[..]);
    let decoded = codec.decode(&mut bytes);
    assert!(decoded.is_err());

    //Encode and try decode
    let mut bytes = BytesMut::new();
    let message = RawNetworkMessage::new(Magic::BITCOIN, NetworkMessage::Verack);
    assert!(codec.encode(&message, &mut bytes).is_ok());
    let decoded = codec.decode(&mut bytes);
    assert_eq!(decoded.unwrap().unwrap(), message);
}
