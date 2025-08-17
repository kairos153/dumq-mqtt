//! # Publish Packet Codec
//! 
//! This module handles the encoding and decoding of MQTT publish-related packets:
//! - Publish: Message publishing with topic and payload
//! - PubAck: Acknowledgment for QoS 1 messages
//! - PubRec: Receipt for QoS 2 messages
//! - PubRel: Release for QoS 2 messages
//! - PubComp: Completion for QoS 2 messages

use crate::error::Result;
use crate::types::{PacketPayload, PublishPacket, PubAckPacket, PubRecPacket, PubRelPacket, PubCompPacket};
use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::utils::{encode_string, decode_string};
use super::properties::{encode_publish_properties, decode_publish_properties};

/// Encode Publish packet payload
pub fn encode_publish(publish: &PublishPacket, buf: &mut BytesMut, protocol_version: u8) -> Result<()> {
    // Topic name
    encode_string(&publish.topic_name, buf)?;
    
    // Packet ID (for QoS > 0)
    if publish.packet_id.is_some() {
        buf.put_u16(publish.packet_id.unwrap());
    }
    
    // MQTT 5.0 properties
    if protocol_version == 5 {
        if let Some(ref properties) = publish.properties {
            encode_publish_properties(properties, buf)?;
        }
    }
    
    // Payload
    buf.put_slice(&publish.payload);
    
    Ok(())
}

/// Decode Publish packet payload
pub fn decode_publish(header: &crate::types::PacketHeader, buf: &mut BytesMut, protocol_version: u8) -> Result<PacketPayload> {
    // Topic name
    let topic_name = decode_string(buf)?;
    
    // Packet ID (for QoS > 0)
    let packet_id = if header.qos > 0 {
        Some(buf.get_u16())
    } else {
        None
    };
    
    // MQTT 5.0 properties
    let properties = if protocol_version == 5 {
        Some(decode_publish_properties(buf)?)
    } else {
        None
    };
    
    // Payload (remaining bytes)
    let payload = if buf.is_empty() {
        Bytes::new()
    } else {
        buf.split_to(buf.len()).freeze()
    };
    
    let publish = PublishPacket {
        topic_name,
        packet_id,
        payload,
        properties,
    };
    
    Ok(PacketPayload::Publish(publish))
}

/// Encode PubAck packet payload
pub fn encode_puback(puback: &PubAckPacket, buf: &mut BytesMut, protocol_version: u8) -> Result<()> {
    // Packet ID
    buf.put_u16(puback.packet_id);
    
    // Reason code (MQTT 5.0)
    if protocol_version == 5 {
        if let Some(reason_code) = puback.reason_code {
            buf.put_u8(reason_code);
        }
        
        // Properties (MQTT 5.0) - placeholder for now
        if let Some(_properties) = &puback.properties {
            // TODO: Implement properties encoding
        }
    }
    
    Ok(())
}

/// Decode PubAck packet payload
pub fn decode_puback(buf: &mut BytesMut, protocol_version: u8) -> Result<PacketPayload> {
    // Packet ID
    let packet_id = buf.get_u16();
    
    // Reason code (MQTT 5.0) - default to 0 (Success)
    let reason_code = if protocol_version == 5 && buf.has_remaining() {
        Some(buf.get_u8())
    } else {
        None
    };
    
    // Properties (MQTT 5.0) - placeholder for now
    let properties = None;
    
    Ok(PacketPayload::PubAck(PubAckPacket {
        packet_id,
        reason_code,
        properties,
    }))
}

/// Encode PubRec packet payload
pub fn encode_pubrec(pubrec: &PubRecPacket, buf: &mut BytesMut, protocol_version: u8) -> Result<()> {
    // Packet ID
    buf.put_u16(pubrec.packet_id);
    
    // Reason code (MQTT 5.0)
    if protocol_version == 5 {
        if let Some(reason_code) = pubrec.reason_code {
            buf.put_u8(reason_code);
        }
        
        // Properties (MQTT 5.0) - placeholder for now
        if let Some(_properties) = &pubrec.properties {
            // TODO: Implement properties encoding
        }
    }
    
    Ok(())
}

/// Decode PubRec packet payload
pub fn decode_pubrec(buf: &mut BytesMut, protocol_version: u8) -> Result<PacketPayload> {
    // Packet ID
    let packet_id = buf.get_u16();
    
    // Reason code (MQTT 5.0) - default to 0 (Success)
    let reason_code = if protocol_version == 5 && buf.has_remaining() {
        Some(buf.get_u8())
    } else {
        None
    };
    
    // Properties (MQTT 5.0) - placeholder for now
    let properties = None;
    
    Ok(PacketPayload::PubRec(PubRecPacket {
        packet_id,
        reason_code,
        properties,
    }))
}

/// Encode PubRel packet payload
pub fn encode_pubrel(pubrel: &PubRelPacket, buf: &mut BytesMut, protocol_version: u8) -> Result<()> {
    // Packet ID
    buf.put_u16(pubrel.packet_id);
    
    // Reason code (MQTT 5.0)
    if protocol_version == 5 {
        if let Some(reason_code) = pubrel.reason_code {
            buf.put_u8(reason_code);
        }
        
        // Properties (MQTT 5.0) - placeholder for now
        if let Some(_properties) = &pubrel.properties {
            // TODO: Implement properties encoding
        }
    }
    
    Ok(())
}

/// Decode PubRel packet payload
pub fn decode_pubrel(buf: &mut BytesMut, protocol_version: u8) -> Result<PacketPayload> {
    // Packet ID
    let packet_id = buf.get_u16();
    
    // Reason code (MQTT 5.0) - default to 0 (Success)
    let reason_code = if protocol_version == 5 && buf.has_remaining() {
        Some(buf.get_u8())
    } else {
        None
    };
    
    // Properties (MQTT 5.0) - placeholder for now
    let properties = None;
    
    Ok(PacketPayload::PubRel(PubRelPacket {
        packet_id,
        reason_code,
        properties,
    }))
}

/// Encode PubComp packet payload
pub fn encode_pubcomp(pubcomp: &PubCompPacket, buf: &mut BytesMut, protocol_version: u8) -> Result<()> {
    // Packet ID
    buf.put_u16(pubcomp.packet_id);
    
    // Reason code (MQTT 5.0)
    if protocol_version == 5 {
        if let Some(reason_code) = pubcomp.reason_code {
            buf.put_u8(reason_code);
        }
        
        // Properties (MQTT 5.0) - placeholder for now
        if let Some(_properties) = &pubcomp.properties {
            // TODO: Implement properties encoding
        }
    }
    
    Ok(())
}

/// Decode PubComp packet payload
pub fn decode_pubcomp(buf: &mut BytesMut, protocol_version: u8) -> Result<PacketPayload> {
    // Packet ID
    let packet_id = buf.get_u16();
    
    // Reason code (MQTT 5.0) - default to 0 (Success)
    let reason_code = if protocol_version == 5 && buf.has_remaining() {
        Some(buf.get_u8())
    } else {
        None
    };
    
    // Properties (MQTT 5.0) - placeholder for now
    let properties = None;
    
    Ok(PacketPayload::PubComp(PubCompPacket {
        packet_id,
        reason_code,
        properties,
    }))
}
