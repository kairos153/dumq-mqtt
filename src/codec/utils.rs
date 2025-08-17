//! # Codec Utilities
//! 
//! This module provides utility functions for encoding and decoding common MQTT data types
//! such as strings, bytes, and remaining length values.

use crate::error::{Error, Result};
use bytes::{Buf, BufMut, Bytes, BytesMut};

/// Encode a UTF-8 string with length prefix
pub fn encode_string(s: &str, buf: &mut BytesMut) -> Result<()> {
    let bytes = s.as_bytes();
    if bytes.len() > 65535 {
        return Err(Error::InvalidPacket("String too long".to_string()));
    }
    buf.put_u16(bytes.len() as u16);
    buf.put_slice(bytes);
    Ok(())
}

/// Decode a UTF-8 string with length prefix
pub fn decode_string(buf: &mut BytesMut) -> Result<String> {
    if buf.len() < 2 {
        return Err(Error::InvalidPacket("Insufficient data for string length".to_string()));
    }
    let len = buf.get_u16() as usize;
    log::debug!("Decoding string with length: {}", len);
    if buf.len() < len {
        return Err(Error::InvalidPacket(format!("Insufficient data for string: need {}, have {}", len, buf.len())));
    }
    let bytes = buf.split_to(len);
    String::from_utf8(bytes.to_vec())
        .map_err(|e| Error::InvalidPacket(format!("Invalid UTF-8: {}", e)))
}

/// Encode binary data with length prefix
pub fn encode_bytes(data: &[u8], buf: &mut BytesMut) -> Result<()> {
    if data.len() > 65535 {
        return Err(Error::InvalidPacket("Data too long".to_string()));
    }
    buf.put_u16(data.len() as u16);
    buf.put_slice(data);
    Ok(())
}

/// Decode binary data with length prefix
pub fn decode_bytes(buf: &mut BytesMut) -> Result<Bytes> {
    let len = buf.get_u16() as usize;
    if buf.len() < len {
        return Err(Error::InvalidPacket("Insufficient data for bytes".to_string()));
    }
    Ok(buf.split_to(len).freeze())
}

/// Encode MQTT remaining length field
/// 
/// The remaining length is encoded using a variable-length encoding scheme:
/// - Each byte encodes 7 bits of the length value
/// - The most significant bit (bit 7) indicates if there are more bytes
/// - Maximum value is 268,435,455 (0xFFFFFF7F)
pub fn encode_remaining_length(len: usize, buf: &mut BytesMut) -> Result<()> {
    if len > 268_435_455 {
        return Err(Error::InvalidPacket("Remaining length too large".to_string()));
    }
    
    let mut value = len;
    loop {
        let mut byte = (value % 128) as u8;
        value /= 128;
        if value > 0 {
            byte |= 0x80;
        }
        buf.put_u8(byte);
        if value == 0 {
            break;
        }
    }
    Ok(())
}

/// Decode MQTT remaining length field
/// 
/// Decodes the variable-length remaining length field according to MQTT specification.
/// Returns the decoded length value or an error if the data is malformed.
pub fn decode_remaining_length(buf: &mut BytesMut) -> Result<usize> {
    let mut value = 0usize;
    let mut multiplier = 1usize;
    
    log::debug!("Decoding remaining length, buffer size: {}", buf.len());
    
    for i in 0..4 {
        if buf.is_empty() {
            return Err(Error::InvalidPacket("Insufficient data for remaining length".to_string()));
        }
        
        let byte = buf[0];
        buf.advance(1);
        
        log::debug!("Remaining length byte {}: 0x{:02x}", i, byte);
        
        value += ((byte & 0x7F) as usize) * multiplier;
        multiplier *= 128;
        
        if (byte & 0x80) == 0 {
            break;
        }
    }
    
    log::debug!("Decoded remaining length: {}", value);
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_string() {
        let mut buf = BytesMut::new();
        
        // Test encoding
        encode_string("test", &mut buf).unwrap();
        assert_eq!(buf.len(), 6); // 2 bytes for length + 4 bytes for string
        
        // Test decoding
        let decoded = decode_string(&mut buf).unwrap();
        assert_eq!(decoded, "test");
    }

    #[test]
    fn test_encode_decode_bytes() {
        let mut buf = BytesMut::new();
        let data = b"test data";
        
        // Test encoding
        encode_bytes(data, &mut buf).unwrap();
        assert_eq!(buf.len(), 11); // 2 bytes for length + 9 bytes for data
        
        // Test decoding
        let decoded = decode_bytes(&mut buf).unwrap();
        assert_eq!(decoded, Bytes::from(data.as_ref()));
    }

    #[test]
    fn test_encode_decode_remaining_length() {
        let mut buf = BytesMut::new();
        
        // Test small values
        encode_remaining_length(0, &mut buf).unwrap();
        assert_eq!(buf.len(), 1);
        assert_eq!(decode_remaining_length(&mut buf).unwrap(), 0);
        
        // Test medium values
        buf.clear();
        encode_remaining_length(127, &mut buf).unwrap();
        assert_eq!(buf.len(), 1);
        assert_eq!(decode_remaining_length(&mut buf).unwrap(), 127);
        
        // Test larger values
        buf.clear();
        encode_remaining_length(128, &mut buf).unwrap();
        assert_eq!(buf.len(), 2);
        assert_eq!(decode_remaining_length(&mut buf).unwrap(), 128);
        
        // Test maximum value
        buf.clear();
        encode_remaining_length(268_435_455, &mut buf).unwrap();
        assert_eq!(buf.len(), 4);
        assert_eq!(decode_remaining_length(&mut buf).unwrap(), 268_435_455);
    }

    #[test]
    fn test_invalid_remaining_length() {
        let mut buf = BytesMut::new();
        
        // Test too large value
        let result = encode_remaining_length(268_435_456, &mut buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_string_length() {
        let mut buf = BytesMut::new();
        
        // Test string too long
        let long_string = "a".repeat(65536);
        let result = encode_string(&long_string, &mut buf);
        assert!(result.is_err());
    }
}
