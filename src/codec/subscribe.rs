//! # Subscribe Packet Codec
//! 
//! This module handles the encoding and decoding of MQTT subscription-related packets:
//! - Subscribe: Client subscription requests
//! - SubAck: Server subscription acknowledgments
//! - Unsubscribe: Client unsubscription requests
//! - UnsubAck: Server unsubscription acknowledgments

use crate::error::Result;
use crate::types::{PacketPayload, SubscribePacket, SubAckPacket, UnsubscribePacket, UnsubAckPacket, TopicFilter};
use bytes::{Buf, BufMut, BytesMut};

use super::utils::{encode_string, decode_string};

/// Encode Subscribe packet payload
pub fn encode_subscribe(subscribe: &SubscribePacket, buf: &mut BytesMut, _protocol_version: u8) -> Result<()> {
    // Packet ID
    buf.put_u16(subscribe.packet_id);
    
    // Topic filters
    for topic_filter in &subscribe.topic_filters {
        encode_string(&topic_filter.topic, buf)?;
        buf.put_u8(topic_filter.qos);
    }
    
    // MQTT 5.0 properties - placeholder for now
    if let Some(_properties) = &subscribe.properties {
        // TODO: Implement properties encoding
    }
    
    Ok(())
}

/// Decode Subscribe packet payload
pub fn decode_subscribe(buf: &mut BytesMut, _protocol_version: u8) -> Result<PacketPayload> {
    // Packet ID
    let packet_id = buf.get_u16();
    
    // Topic filters
    let mut topic_filters = Vec::new();
    while buf.has_remaining() {
        let topic = decode_string(buf)?;
        let qos = buf.get_u8();
        
        topic_filters.push(TopicFilter {
            topic,
            qos,
            no_local: false,
            retain_as_published: false,
            retain_handling: 0,
        });
    }
    
    // MQTT 5.0 properties - placeholder for now
    let properties = None;
    
    Ok(PacketPayload::Subscribe(SubscribePacket {
        packet_id,
        topic_filters,
        properties,
    }))
}

/// Encode SubAck packet payload
pub fn encode_suback(suback: &SubAckPacket, buf: &mut BytesMut, _protocol_version: u8) -> Result<()> {
    // Packet ID
    buf.put_u16(suback.packet_id);
    
    // Return codes
    for &return_code in &suback.return_codes {
        buf.put_u8(return_code);
    }
    
    // MQTT 5.0 properties - placeholder for now
    if let Some(_properties) = &suback.properties {
        // TODO: Implement properties encoding
    }
    
    Ok(())
}

/// Decode SubAck packet payload
pub fn decode_suback(buf: &mut BytesMut, _protocol_version: u8) -> Result<PacketPayload> {
    // Packet ID
    let packet_id = buf.get_u16();
    
    // Return codes
    let mut return_codes = Vec::new();
    while buf.has_remaining() {
        return_codes.push(buf.get_u8());
    }
    
    // MQTT 5.0 properties - placeholder for now
    let properties = None;
    
    Ok(PacketPayload::SubAck(SubAckPacket {
        packet_id,
        return_codes,
        properties,
    }))
}

/// Encode Unsubscribe packet payload
pub fn encode_unsubscribe(unsubscribe: &UnsubscribePacket, buf: &mut BytesMut, _protocol_version: u8) -> Result<()> {
    // Packet ID
    buf.put_u16(unsubscribe.packet_id);
    
    // Topic filters
    for topic_filter in &unsubscribe.topic_filters {
        encode_string(topic_filter, buf)?;
    }
    
    // MQTT 5.0 properties - placeholder for now
    if let Some(_properties) = &unsubscribe.properties {
        // TODO: Implement properties encoding
    }
    
    Ok(())
}

/// Decode Unsubscribe packet payload
pub fn decode_unsubscribe(buf: &mut BytesMut, _protocol_version: u8) -> Result<PacketPayload> {
    // Packet ID
    let packet_id = buf.get_u16();
    
    // Topic filters
    let mut topic_filters = Vec::new();
    while buf.has_remaining() {
        topic_filters.push(decode_string(buf)?);
    }
    
    // MQTT 5.0 properties - placeholder for now
    let properties = None;
    
    Ok(PacketPayload::Unsubscribe(UnsubscribePacket {
        packet_id,
        topic_filters,
        properties,
    }))
}

/// Encode UnsubAck packet payload
pub fn encode_unsuback(unsuback: &UnsubAckPacket, buf: &mut BytesMut, _protocol_version: u8) -> Result<()> {
    // Packet ID
    buf.put_u16(unsuback.packet_id);
    
    // Reason codes (MQTT 5.0)
    if _protocol_version == 5 {
        for &reason_code in &unsuback.reason_codes {
            buf.put_u8(reason_code);
        }
    }
    
    // MQTT 5.0 properties - placeholder for now
    if let Some(_properties) = &unsuback.properties {
        // TODO: Implement properties encoding
    }
    
    Ok(())
}

/// Decode UnsubAck packet payload
pub fn decode_unsuback(buf: &mut BytesMut, protocol_version: u8) -> Result<PacketPayload> {
    // Packet ID
    let packet_id = buf.get_u16();
    
    // Reason codes (MQTT 5.0) - empty for MQTT 3.1.1
    let reason_codes = if protocol_version == 5 {
        let mut codes = Vec::new();
        while buf.has_remaining() {
            codes.push(buf.get_u8());
        }
        codes
    } else {
        Vec::new()
    };
    
    // MQTT 5.0 properties - placeholder for now
    let properties = None;
    
    Ok(PacketPayload::UnsubAck(UnsubAckPacket {
        packet_id,
        reason_codes,
        properties,
    }))
}
