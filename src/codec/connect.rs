//! # Connect and ConnAck Packet Codec
//! 
//! This module handles the encoding and decoding of MQTT Connect and ConnAck packets,
//! including MQTT 5.0 properties support.

use crate::error::{Error, Result};
use crate::types::{PacketPayload, ConnectPacket, ConnAckPacket, ConnectReturnCode};
use bytes::{Buf, BufMut, BytesMut};

use super::utils::{encode_string, encode_bytes, decode_string, decode_bytes};
use super::properties::{encode_connect_properties, decode_connect_properties, encode_connack_properties, decode_connack_properties};

/// Encode Connect packet payload
pub fn encode_connect(connect: &ConnectPacket, buf: &mut BytesMut, protocol_version: u8) -> Result<()> {
    // Protocol name
    encode_string(&connect.protocol_name, buf)?;
    
    // Protocol version
    buf.put_u8(connect.protocol_version);
    
    // Connect flags
    let mut connect_flags = 0u8;
    if connect.clean_session {
        connect_flags |= 0x02;
    }
    if connect.will_flag {
        connect_flags |= 0x04;
        connect_flags |= (connect.will_qos & 0x03) << 3;
        if connect.will_retain {
            connect_flags |= 0x20;
        }
    }
    if connect.password_flag {
        connect_flags |= 0x40;
    }
    if connect.username_flag {
        connect_flags |= 0x80;
    }
    buf.put_u8(connect_flags);
    
    // Keep alive
    buf.put_u16(connect.keep_alive);
    
    // Client ID
    encode_string(&connect.client_id, buf)?;
    
    // Will topic and message
    if connect.will_flag {
        if let Some(ref will_topic) = connect.will_topic {
            encode_string(will_topic, buf)?;
        }
        if let Some(ref will_message) = connect.will_message {
            encode_bytes(will_message, buf)?;
        }
    }
    
    // Username and password
    if connect.username_flag {
        if let Some(ref username) = connect.username {
            encode_string(username, buf)?;
        }
    }
    if connect.password_flag {
        if let Some(ref password) = connect.password {
            encode_string(password, buf)?;
        }
    }
    
    // MQTT 5.0 properties
    if protocol_version == 5 {
        if let Some(ref properties) = connect.properties {
            encode_connect_properties(properties, buf)?;
        }
    }
    
    Ok(())
}

/// Decode Connect packet payload
pub fn decode_connect(buf: &mut BytesMut, protocol_version: u8) -> Result<PacketPayload> {
    // Protocol name
    let protocol_name = decode_string(buf)?;
    
    // Protocol version
    let protocol_version_decoded = buf.get_u8();
    
    // Connect flags
    let connect_flags = buf.get_u8();
    let clean_session = (connect_flags & 0x02) != 0;
    let will_flag = (connect_flags & 0x04) != 0;
    let will_qos = (connect_flags & 0x18) >> 3;
    let will_retain = (connect_flags & 0x20) != 0;
    let password_flag = (connect_flags & 0x40) != 0;
    let username_flag = (connect_flags & 0x80) != 0;
    
    // Keep alive
    let keep_alive = buf.get_u16();
    
    // Client ID
    let client_id = decode_string(buf)?;
    
    // Will topic and message
    let will_topic = if will_flag {
        Some(decode_string(buf)?)
    } else {
        None
    };
    
    let will_message = if will_flag {
        Some(decode_bytes(buf)?)
    } else {
        None
    };
    
    // Username and password
    let username = if username_flag {
        Some(decode_string(buf)?)
    } else {
        None
    };
    
    let password = if password_flag {
        let password_bytes = decode_bytes(buf)?;
        Some(String::from_utf8(password_bytes.to_vec())
            .map_err(|e| Error::InvalidPacket(format!("Invalid UTF-8 in password: {}", e)))?)
    } else {
        None
    };
    
    // MQTT 5.0 properties
    let properties = if protocol_version == 5 {
        Some(decode_connect_properties(buf)?)
    } else {
        None
    };
    
    let connect = ConnectPacket {
        protocol_name,
        protocol_version: protocol_version_decoded,
        clean_session,
        will_flag,
        will_qos,
        will_retain,
        password_flag,
        username_flag,
        keep_alive,
        client_id,
        will_topic,
        will_message,
        username,
        password,
        properties,
    };
    
    Ok(PacketPayload::Connect(connect))
}

/// Encode ConnAck packet payload
pub fn encode_connack(connack: &ConnAckPacket, buf: &mut BytesMut, protocol_version: u8) -> Result<()> {
    // Connect acknowledge flags
    let mut ack_flags = 0u8;
    if connack.session_present {
        ack_flags |= 0x01;
    }
    buf.put_u8(ack_flags);
    
    // Return code
    buf.put_u8(connack.return_code as u8);
    
    // MQTT 5.0 properties
    if protocol_version == 5 {
        if let Some(ref properties) = connack.properties {
            encode_connack_properties(properties, buf)?;
        }
    }
    
    Ok(())
}

/// Decode ConnAck packet payload
pub fn decode_connack(buf: &mut BytesMut, protocol_version: u8) -> Result<PacketPayload> {
    // Connect acknowledge flags
    let ack_flags = buf.get_u8();
    let session_present = (ack_flags & 0x01) != 0;
    
    // Return code
    let return_code = ConnectReturnCode::from_u8(buf.get_u8())
        .ok_or_else(|| Error::InvalidPacket("Invalid connect return code".to_string()))?;
    
    // MQTT 5.0 properties
    let properties = if protocol_version == 5 {
        Some(decode_connack_properties(buf)?)
    } else {
        None
    };
    
    let connack = ConnAckPacket {
        session_present,
        return_code,
        properties,
    };
    
    Ok(PacketPayload::ConnAck(connack))
}
