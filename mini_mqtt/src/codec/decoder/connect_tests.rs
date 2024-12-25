
use super::connect::*;
use crate::packets::{FixedHeader, Bits, TwoByteInteger, UTF8EncodedString, VariableByteInteger, UTF8StringPair};
use crate::packets::connect::ConnectFlags;

#[test]
fn connect_parser_valid_input() {
    let input = vec![
        0x00, 0x04, b'M', b'Q', b'T', b'T', // Protocol Name
        0x05, // Protocol Version
        0x02, // Connect Flags
        0x00, 0x3C, // Keep Alive
        0x00, // Properties Length
        0x00, 0x04, b't', b'e', b's', b't', // Client ID
    ];
    let fixed_header = FixedHeader::new(Bits(0x10), Bits(0x00), VariableByteInteger(input.len() as u32)).unwrap();

    let result = connect_parser(fixed_header)(&input);
    let (_, connect) = result.unwrap();
    assert_eq!(connect.variable_header.protocol_name,  UTF8EncodedString( "MQTT".to_string()));
    assert_eq!(connect.variable_header.protocol_version, Bits(5));
    assert_eq!(connect.variable_header.connect_flags, ConnectFlags(Bits(0x02)));
    assert_eq!(connect.variable_header.keep_alive, TwoByteInteger(60));
    assert!(connect.variable_header.properties.is_empty());
    assert_eq!(connect.payload.client_id, UTF8EncodedString("test".to_string()));
}

#[test]
fn connect_parser_missing_client_id() {
    let input = vec![
        0x00, 0x04, b'M', b'Q', b'T', b'T', // Protocol Name
        0x05, // Protocol Version
        0x02, // Connect Flags
        0x00, 0x3C, // Keep Alive
        0x00, // Properties Length
    ];
    let fixed_header = FixedHeader::new(Bits(0x10), Bits(0x00), VariableByteInteger(input.len() as u32)).unwrap();

    let result = connect_parser(fixed_header)(&input);
    assert!(result.is_err());
}

#[test]
fn parse_variable_header_valid_input() {
    let input = vec![
        0x00, 0x04, b'M', b'Q', b'T', b'T', // Protocol Name
        0x05, // Protocol Version
        0x02, // Connect Flags
        0x00, 0x3C, // Keep Alive
        0x00, // Properties Length
    ];
    let result = parse_variable_header(&input);
    assert!(result.is_ok());
    let (_, variable_header) = result.unwrap();
    assert_eq!(variable_header.protocol_name, UTF8EncodedString( "MQTT".to_string()));
    assert_eq!(variable_header.protocol_version, Bits(5));
    assert_eq!(variable_header.connect_flags, ConnectFlags(Bits(0x02)));
    assert_eq!(variable_header.keep_alive, TwoByteInteger(60));
    assert!(variable_header.properties.is_empty());
}

#[test]
fn parse_payload_valid_input() {
    let connect_flags = ConnectFlags::new(Bits(0x02)).unwrap();
    let input = vec![
        0x00, 0x04, b't', b'e', b's', b't', // Client ID
    ];
    let result = parse_payload(&input, connect_flags);
    let (_, payload) = result.unwrap();
    assert_eq!(payload.client_id, UTF8EncodedString("test".to_string()));
    assert!(payload.will_properties.is_none());
    assert!(payload.will_topic.is_none());
    assert!(payload.will_payload.is_none());
    assert!(payload.user_name.is_none());
    assert!(payload.password.is_none());
}

#[test]
fn parse_payload_with_will_flag() {
    let connect_flags = ConnectFlags::new(Bits(0xE4)).unwrap(); // User Name, Password,  and Will Flag set
    let input = vec![
        0x00, 0x04, b't', b'e', b's', b't', // Client ID
        0x0E, // Will Properties Length
        0x26, // User Property of Will Properties
        0x00, 0x04, b'u', b's', b'e', b'r', // user
        0x00, 0x05, b'v', b'a', b'l', b'u', b'e', // value
        0x00, 0x04, b'w', b'i', b'l', b'l', // Will Topic
        0x00, 0x03, b'd', b'a', b't', // Will Payload
        0x00, 0x08, b'u', b's', b'e', b'r', b'n', b'a', b'm', b'e', // User Name
        0x00, 0x03, b'p', b'a', b's', // Password
    ];
    let result = parse_payload(&input, connect_flags);
    assert!(result.is_ok());
    let (_, payload) = result.unwrap();
    assert_eq!(payload.client_id, UTF8EncodedString("test".to_string()));
    let properties = payload.will_properties.unwrap();
    assert_eq!(properties.len(), 1);
    let user_property = properties.get_as::<UTF8StringPair>(VariableByteInteger(0x26)).unwrap();
    assert_eq!(user_property, Some(&UTF8StringPair("user".to_string(), "value".to_string())));
    assert_eq!(payload.will_topic.unwrap(), UTF8EncodedString("will".to_string()));
    assert_eq!(payload.will_payload.unwrap().val(), &vec![b'd', b'a', b't']);
    assert_eq!(payload.user_name.unwrap(), UTF8EncodedString("username".to_string()));
    assert_eq!(payload.password.unwrap().val(), &vec![b'p', b'a', b's']);
}