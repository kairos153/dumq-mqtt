//! # MQTT 5.0 Properties Codec
//! 
//! This module handles the encoding and decoding of MQTT 5.0 properties
//! for various packet types including Connect, ConnAck, and Publish packets.

use crate::error::Result;
use crate::types::{ConnectProperties, ConnAckProperties, PublishProperties};
use bytes::{Buf, BufMut, BytesMut};
use std::collections::HashMap;

use super::utils::{encode_string, encode_bytes, decode_string, decode_bytes};

/// Encode Connect packet properties
pub fn encode_connect_properties(properties: &ConnectProperties, buf: &mut BytesMut) -> Result<()> {
    // Properties length (will be calculated)
    let properties_start = buf.len();
    buf.put_u8(0); // Placeholder for properties length
    
    // Session Expiry Interval (0x11)
    if let Some(session_expiry) = properties.session_expiry_interval {
        buf.put_u8(0x11);
        buf.put_u32(session_expiry);
    }
    
    // Receive Maximum (0x21)
    if let Some(receive_max) = properties.receive_maximum {
        buf.put_u8(0x21);
        buf.put_u16(receive_max);
    }
    
    // Maximum Packet Size (0x27)
    if let Some(max_packet_size) = properties.max_packet_size {
        buf.put_u8(0x27);
        buf.put_u32(max_packet_size);
    }
    
    // Topic Alias Maximum (0x22)
    if let Some(topic_alias_max) = properties.topic_alias_maximum {
        buf.put_u8(0x22);
        buf.put_u16(topic_alias_max);
    }
    
    // Request Response Information (0x19)
    if let Some(request_response_info) = properties.request_response_information {
        buf.put_u8(0x19);
        buf.put_u8(if request_response_info { 1 } else { 0 });
    }
    
    // Request Problem Information (0x17)
    if let Some(request_problem_info) = properties.request_problem_information {
        buf.put_u8(0x17);
        buf.put_u8(if request_problem_info { 1 } else { 0 });
    }
    
    // User Properties (0x26)
    for (key, value) in &properties.user_properties {
        buf.put_u8(0x26);
        encode_string(key, buf)?;
        encode_string(value, buf)?;
    }
    
    // Authentication Method (0x15)
    if let Some(ref auth_method) = properties.authentication_method {
        buf.put_u8(0x15);
        encode_string(auth_method, buf)?;
    }
    
    // Authentication Data (0x16)
    if let Some(ref auth_data) = properties.authentication_data {
        buf.put_u8(0x16);
        encode_bytes(auth_data, buf)?;
    }
    
    // Update properties length
    let properties_len = buf.len() - properties_start - 1;
    if properties_len > 0 {
        buf[properties_start] = properties_len as u8;
    }
    
    Ok(())
}

/// Decode Connect packet properties
pub fn decode_connect_properties(buf: &mut BytesMut) -> Result<ConnectProperties> {
    let mut properties = ConnectProperties {
        session_expiry_interval: None,
        receive_maximum: None,
        max_packet_size: None,
        topic_alias_maximum: None,
        request_response_information: None,
        request_problem_information: None,
        user_properties: HashMap::new(),
        authentication_method: None,
        authentication_data: None,
    };
    
    // Properties length
    let properties_length = buf.get_u8() as usize;
    if properties_length == 0 {
        return Ok(properties);
    }
    
    let _properties_end = buf.len() - properties_length;
    let mut properties_buf = buf.split_to(properties_length);
    
    while properties_buf.has_remaining() {
        let property_id = properties_buf.get_u8();
        
        match property_id {
            0x11 => { // Session Expiry Interval
                properties.session_expiry_interval = Some(properties_buf.get_u32());
            }
            0x21 => { // Receive Maximum
                properties.receive_maximum = Some(properties_buf.get_u16());
            }
            0x27 => { // Maximum Packet Size
                properties.max_packet_size = Some(properties_buf.get_u32());
            }
            0x22 => { // Topic Alias Maximum
                properties.topic_alias_maximum = Some(properties_buf.get_u16());
            }
            0x19 => { // Request Response Information
                properties.request_response_information = Some(properties_buf.get_u8() != 0);
            }
            0x17 => { // Request Problem Information
                properties.request_problem_information = Some(properties_buf.get_u8() != 0);
            }
            0x26 => { // User Properties
                let key = decode_string(&mut properties_buf)?;
                let value = decode_string(&mut properties_buf)?;
                properties.user_properties.insert(key, value);
            }
            0x15 => { // Authentication Method
                properties.authentication_method = Some(decode_string(&mut properties_buf)?);
            }
            0x16 => { // Authentication Data
                properties.authentication_data = Some(decode_bytes(&mut properties_buf)?);
            }
            _ => {
                // Unknown property, skip it
                log::warn!("Unknown connect property ID: 0x{:02x}", property_id);
                // Try to skip the property value (this is a simplified approach)
                if properties_buf.has_remaining() {
                    properties_buf.advance(1);
                }
            }
        }
    }
    
    Ok(properties)
}

/// Encode ConnAck packet properties
pub fn encode_connack_properties(properties: &ConnAckProperties, buf: &mut BytesMut) -> Result<()> {
    // Properties length (will be calculated)
    let properties_start = buf.len();
    buf.put_u8(0); // Placeholder for properties length
    
    // Session Expiry Interval (0x11)
    if let Some(session_expiry) = properties.session_expiry_interval {
        buf.put_u8(0x11);
        buf.put_u32(session_expiry);
    }
    
    // Receive Maximum (0x21)
    if let Some(receive_max) = properties.receive_maximum {
        buf.put_u8(0x21);
        buf.put_u16(receive_max);
    }
    
    // Maximum QoS (0x24)
    if let Some(max_qos) = properties.max_qos {
        buf.put_u8(0x24);
        buf.put_u8(max_qos);
    }
    
    // Retain Available (0x25)
    if let Some(retain_available) = properties.retain_available {
        buf.put_u8(0x25);
        buf.put_u8(if retain_available { 1 } else { 0 });
    }
    
    // Maximum Packet Size (0x27)
    if let Some(max_packet_size) = properties.max_packet_size {
        buf.put_u8(0x27);
        buf.put_u32(max_packet_size);
    }
    
    // Assigned Client Identifier (0x12)
    if let Some(ref assigned_client_id) = properties.assigned_client_identifier {
        buf.put_u8(0x12);
        encode_string(assigned_client_id, buf)?;
    }
    
    // Topic Alias Maximum (0x22)
    if let Some(topic_alias_max) = properties.topic_alias_maximum {
        buf.put_u8(0x22);
        buf.put_u16(topic_alias_max);
    }
    
    // Reason String (0x1F)
    if let Some(ref reason_string) = properties.reason_string {
        buf.put_u8(0x1F);
        encode_string(reason_string, buf)?;
    }
    
    // User Properties (0x26)
    for (key, value) in &properties.user_properties {
        buf.put_u8(0x26);
        encode_string(key, buf)?;
        encode_string(value, buf)?;
    }
    
    // Wildcard Subscription Available (0x28)
    if let Some(wildcard_sub_available) = properties.wildcard_subscription_available {
        buf.put_u8(0x28);
        buf.put_u8(if wildcard_sub_available { 1 } else { 0 });
    }
    
    // Subscription Identifiers Available (0x29)
    if let Some(sub_id_available) = properties.subscription_identifiers_available {
        buf.put_u8(0x29);
        buf.put_u8(if sub_id_available { 1 } else { 0 });
    }
    
    // Shared Subscription Available (0x2A)
    if let Some(shared_sub_available) = properties.shared_subscription_available {
        buf.put_u8(0x2A);
        buf.put_u8(if shared_sub_available { 1 } else { 0 });
    }
    
    // Server Keep Alive (0x13)
    if let Some(server_keep_alive) = properties.server_keep_alive {
        buf.put_u8(0x13);
        buf.put_u16(server_keep_alive);
    }
    
    // Response Information (0x1A)
    if let Some(ref response_info) = properties.response_information {
        buf.put_u8(0x1A);
        encode_string(response_info, buf)?;
    }
    
    // Server Reference (0x1C)
    if let Some(ref server_ref) = properties.server_reference {
        buf.put_u8(0x1C);
        encode_string(server_ref, buf)?;
    }
    
    // Authentication Method (0x15)
    if let Some(ref auth_method) = properties.authentication_method {
        buf.put_u8(0x15);
        encode_string(auth_method, buf)?;
    }
    
    // Authentication Data (0x16)
    if let Some(ref auth_data) = properties.authentication_data {
        buf.put_u8(0x16);
        encode_bytes(auth_data, buf)?;
    }
    
    // Update properties length
    let properties_len = buf.len() - properties_start - 1;
    if properties_len > 0 {
        buf[properties_start] = properties_len as u8;
    }
    
    Ok(())
}

/// Decode ConnAck packet properties
pub fn decode_connack_properties(buf: &mut BytesMut) -> Result<ConnAckProperties> {
    let mut properties = ConnAckProperties {
        session_expiry_interval: None,
        receive_maximum: None,
        max_qos: None,
        retain_available: None,
        max_packet_size: None,
        assigned_client_identifier: None,
        topic_alias_maximum: None,
        reason_string: None,
        user_properties: HashMap::new(),
        wildcard_subscription_available: None,
        subscription_identifiers_available: None,
        shared_subscription_available: None,
        server_keep_alive: None,
        response_information: None,
        server_reference: None,
        authentication_method: None,
        authentication_data: None,
    };
    
    // Properties length
    let properties_length = buf.get_u8() as usize;
    if properties_length == 0 {
        return Ok(properties);
    }
    
    let mut properties_buf = buf.split_to(properties_length);
    
    while properties_buf.has_remaining() {
        let property_id = properties_buf.get_u8();
        
        match property_id {
            0x11 => { // Session Expiry Interval
                properties.session_expiry_interval = Some(properties_buf.get_u32());
            }
            0x21 => { // Receive Maximum
                properties.receive_maximum = Some(properties_buf.get_u16());
            }
            0x24 => { // Maximum QoS
                properties.max_qos = Some(properties_buf.get_u8());
            }
            0x25 => { // Retain Available
                properties.retain_available = Some(properties_buf.get_u8() != 0);
            }
            0x27 => { // Maximum Packet Size
                properties.max_packet_size = Some(properties_buf.get_u32());
            }
            0x12 => { // Assigned Client Identifier
                properties.assigned_client_identifier = Some(decode_string(&mut properties_buf)?);
            }
            0x22 => { // Topic Alias Maximum
                properties.topic_alias_maximum = Some(properties_buf.get_u16());
            }
            0x1F => { // Reason String
                properties.reason_string = Some(decode_string(&mut properties_buf)?);
            }
            0x26 => { // User Properties
                let key = decode_string(&mut properties_buf)?;
                let value = decode_string(&mut properties_buf)?;
                properties.user_properties.insert(key, value);
            }
            0x28 => { // Wildcard Subscription Available
                properties.wildcard_subscription_available = Some(properties_buf.get_u8() != 0);
            }
            0x29 => { // Subscription Identifiers Available
                properties.subscription_identifiers_available = Some(properties_buf.get_u8() != 0);
            }
            0x2A => { // Shared Subscription Available
                properties.shared_subscription_available = Some(properties_buf.get_u8() != 0);
            }
            0x13 => { // Server Keep Alive
                properties.server_keep_alive = Some(properties_buf.get_u16());
            }
            0x1A => { // Response Information
                properties.response_information = Some(decode_string(&mut properties_buf)?);
            }
            0x1C => { // Server Reference
                properties.server_reference = Some(decode_string(&mut properties_buf)?);
            }
            0x15 => { // Authentication Method
                properties.authentication_method = Some(decode_string(&mut properties_buf)?);
            }
            0x16 => { // Authentication Data
                properties.authentication_data = Some(decode_bytes(&mut properties_buf)?);
            }
            _ => {
                // Unknown property, skip it
                log::warn!("Unknown connack property ID: 0x{:02x}", property_id);
                // Try to skip the property value (this is a simplified approach)
                if properties_buf.has_remaining() {
                    properties_buf.advance(1);
                }
            }
        }
    }
    
    Ok(properties)
}

/// Encode Publish packet properties
pub fn encode_publish_properties(_properties: &PublishProperties, _buf: &mut BytesMut) -> Result<()> {
    // TODO: Implement MQTT 5.0 publish properties encoding
    Ok(())
}

/// Decode Publish packet properties
pub fn decode_publish_properties(_buf: &mut BytesMut) -> Result<PublishProperties> {
    // TODO: Implement MQTT 5.0 publish properties decoding
    Ok(PublishProperties {
        payload_format_indicator: None,
        message_expiry_interval: None,
        topic_alias: None,
        response_topic: None,
        correlation_data: None,
        user_properties: HashMap::new(),
        subscription_identifier: None,
        content_type: None,
    })
}
