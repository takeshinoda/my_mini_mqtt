use std::io::Cursor;

use super::super::decoder::*; // the test target
use crate::packets;

#[test]
fn decode_fixed_header_as_connect() {
    let data = vec![0x11, 0x00, 0x80, 0x01];
    let mut cursor = Cursor::new(data);
    let result = decode(&mut cursor);

    let packet = result.unwrap();
    assert!(matches!(&packet, packets::Packet::Connect));
}

#[test]
fn decode_invalid_packet_data() {
    let data = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    let mut cursor = Cursor::new(data);
    let result = decode(&mut cursor);

    assert!(result.is_err());
}

#[test]
fn parse_valid_fixed_header() {
    let data = vec![0x11, 0x80, 0x01];
    let result = parse_fixed_header(&data);

    let (_, fixed_header) = result.unwrap();
    assert_eq!(fixed_header.control_packet_type, Bits(1));
    assert_eq!(fixed_header.flags, Bits(1));
    assert_eq!(fixed_header.remaining_length, VariableByteInteger(128));
}

#[test]
fn parse_valid_fixed_header_first_byte() {
    let data = vec![0x12];
    let result = parse_fixed_header_first_byte(&data);

    let (_, (control_packet_type, flags)) = result.unwrap();
    assert_eq!(control_packet_type, Bits(1));
    assert_eq!(flags, Bits(2));
}

#[test]
fn parse_invalid_fixed_header() {
    let data = vec![];
    let result = parse_fixed_header_first_byte(&data);

    assert!(result.is_err());
}

#[test]
fn parse_two_byte_integer_valid() {
    let data = vec![0x01, 0x02];
    let result = parse_two_byte_integer(&data);
    let (_, two_byte_integer) = result.unwrap();
    assert_eq!(two_byte_integer.val(), 0x0102);
}

#[test]
fn parse_two_byte_integer_invalid_length() {
    let data = vec![0x01];
    let result = parse_two_byte_integer(&data);
    assert!(result.is_err());
}

#[test]
fn parse_four_byte_integer_valid() {
    let data = vec![0x01, 0x02, 0x03, 0x04];
    let result = parse_four_byte_integer(&data);
    assert!(result.is_ok());
    let (_, four_byte_integer) = result.unwrap();
    assert_eq!(four_byte_integer.val(), 0x01020304);
}

#[test]
fn parse_four_byte_integer_invalid_length() {
    let data = vec![0x01, 0x02, 0x03];
    let result = parse_four_byte_integer(&data);
    assert!(result.is_err());
}

#[test]
fn parse_variable_byte_integer_valid() {
    let data = vec![0x7F];
    let result = parse_variable_byte_integer(&data);

    let (_, remaining_length) = result.unwrap();
    assert_eq!(remaining_length, VariableByteInteger(127));
}

#[test]
fn parse_variable_byte_integer_valid2() {
    let data = vec![0x80, 0x80, 0x01];
    let result = parse_variable_byte_integer(&data);

    let (_, remaining_length) = result.unwrap();
    assert_eq!(remaining_length, VariableByteInteger(16384));
}

#[test]
fn parse_variable_byte_integer_valid3() {
    let data = vec![0x80, 0x80, 0x80, 0x01];
    let result = parse_variable_byte_integer(&data);

    let (_, remaining_length) = result.unwrap();
    assert_eq!(remaining_length, VariableByteInteger(2097152));
}

#[test]
fn parse_variable_byte_integer_valid4() {
    let data = vec![0xFF, 0xFF, 0xFF, 0x7F];
    let result = parse_variable_byte_integer(&data);

    let (_, remaining_length) = result.unwrap();
    assert_eq!(remaining_length, VariableByteInteger(268435455));
}

#[test]
fn parse_variable_byte_integer_invalid() {
    let data = vec![0xFF, 0xFF, 0xFF, 0x80];
    let result = parse_variable_byte_integer(&data);

    assert!(result.is_err());
}

#[test]
fn parse_utf8_encoded_string_valid() {
    let data = vec![0x00, 0x05, b'h', b'e', b'l', b'l', b'o'];
    let result = parse_utf8_encoded_string(&data);
    let (_, utf8_string) = result.unwrap();
    assert_eq!(utf8_string.val(), "hello");
}

#[test]
fn parse_utf8_encoded_string_invalid_length() {
    let data = vec![0x00, 0x05, b'h', b'e', b'l'];
    let result = parse_utf8_encoded_string(&data);
    assert!(result.is_err());
}

#[test]
fn parse_utf8_encoded_string_empty() {
    let data = vec![0x00, 0x00];
    let result = parse_utf8_encoded_string(&data);
    let (_, utf8_string) = result.unwrap();
    assert_eq!(utf8_string.val(), "");
}

#[test]
fn parse_utf8_encoded_string_non_utf8() {
    let data = vec![0x00, 0x03, 0xFF, 0xFE, 0xFD];
    let result = parse_utf8_encoded_string(&data);
    let (_, utf8_string) = result.unwrap();
    assert_eq!(utf8_string.val(), "\u{FFFD}\u{FFFD}\u{FFFD}");
}

#[test]
fn parse_utf8_string_pair_valid() {
    let data = vec![0x00, 0x03, b'k', b'e', b'y', 0x00, 0x05, b'v', b'a', b'l', b'u', b'e'];
    let result = parse_utf8_string_pair(&data);
    let (_, utf8_string_pair) = result.unwrap();
    assert_eq!(utf8_string_pair.0, "key");
    assert_eq!(utf8_string_pair.1, "value");
}

#[test]
fn parse_utf8_string_pair_invalid_key_length() {
    let data = vec![0x00, 0x03, b'k', b'e'];
    let result = parse_utf8_string_pair(&data);
    assert!(result.is_err());
}

#[test]
fn parse_utf8_string_pair_invalid_value_length() {
    let data = vec![0x00, 0x03, b'k', b'e', b'y', 0x00, 0x05, b'v', b'a', b'l'];
    let result = parse_utf8_string_pair(&data);
    assert!(result.is_err());
}

#[test]
fn parse_utf8_string_pair_empty_strings() {
    let data = vec![0x00, 0x00, 0x00, 0x00];
    let result = parse_utf8_string_pair(&data);
    let (_, utf8_string_pair) = result.unwrap();
    assert_eq!(utf8_string_pair.0, "");
    assert_eq!(utf8_string_pair.1, "");
}

#[test]
fn parse_utf8_string_pair_non_utf8() {
    let data = vec![0x00, 0x03, 0xFF, 0xFE, 0xFD, 0x00, 0x03, 0xFF, 0xFE, 0xFD];
    let result = parse_utf8_string_pair(&data);
    let (_, utf8_string_pair) = result.unwrap();
    assert_eq!(utf8_string_pair.0, "\u{FFFD}\u{FFFD}\u{FFFD}");
    assert_eq!(utf8_string_pair.1, "\u{FFFD}\u{FFFD}\u{FFFD}");
}

#[test]
fn parse_binary_data_valid() {
    let data = vec![0x00, 0x03, 0x01, 0x02, 0x03];
    let result = parse_binary_data(&data);
    assert!(result.is_ok());
    let (_, binary_data) = result.unwrap();
    let expect  =  vec![0x01, 0x02, 0x03];
    assert_eq!(binary_data.val(), &expect);
}

#[test]
fn parse_binary_data_invalid_length() {
    let data = vec![0x00, 0x03, 0x01];
    let result = parse_binary_data(&data);
    assert!(result.is_err());
}

#[test]
fn parse_binary_data_empty() {
    let data = vec![0x00, 0x00];
    let result = parse_binary_data(&data);
    assert!(result.is_ok());
    let (_, binary_data) = result.unwrap();
    let expect = Vec::<u8>::new();
    assert_eq!(binary_data.val(), &expect);
}

#[test]
fn parse_property_payload_format_indicator() {
    let data = vec![0x01, 0x12];
    let result = parse_property(&data);
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier, PAYLOAD_FORMAT_INDICATOR);
    assert_eq!(value, ValueTypes::Bits(Bits(0x12)));
}

#[test]
fn parse_property_message_expiry_interval() {
    let data = vec![0x02, 0x00, 0x00, 0x00, 0x01];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x02);
    assert_eq!(value, ValueTypes::FourByteInteger(FourByteInteger(0x00000001)));
}

#[test]
fn parse_property_content_type() {
    let data = vec![0x03, 0x00, 0x04, b't', b'e', b's', b't'];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x03);
    assert_eq!(value, ValueTypes::UTF8EncodedString(UTF8EncodedString("test".to_string())));
}

#[test]
fn parse_property_response_topic() {
    let data = vec![0x08, 0x00, 0x04, b't', b'e', b's', b't'];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x08);
    assert_eq!(value, ValueTypes::UTF8EncodedString(UTF8EncodedString("test".to_string())));
}

#[test]
fn parse_property_correlation_data() {
    let data = vec![0x09, 0x00, 0x03, 0x01, 0x02, 0x03];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x09);
    assert_eq!(value, ValueTypes::BinaryData(BinaryData(vec![0x01, 0x02, 0x03])));
}

#[test]
fn parse_property_subscription_identifier() {
    let data = vec![0x0B, 0x01];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x0B);
    assert_eq!(value, ValueTypes::VariableByteInteger(VariableByteInteger(0x01)));
}

#[test]
fn parse_property_session_expiry_interval() {
    let data = vec![0x11, 0x00, 0x00, 0x00, 0x01];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x11);
    assert_eq!(value, ValueTypes::FourByteInteger(FourByteInteger(0x00000001)));
}

#[test]
fn parse_property_assigned_client_identifier() {
    let data = vec![0x12, 0x00, 0x04, b't', b'e', b's', b't'];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x12);
    assert_eq!(value, ValueTypes::UTF8EncodedString(UTF8EncodedString("test".to_string())));
}

#[test]
fn parse_property_server_keep_alive() {
    let data = vec![0x13, 0x00, 0x01];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x13);
    assert_eq!(value, ValueTypes::TwoByteInteger(TwoByteInteger(0x0001)));
}

#[test]
fn parse_property_authentication_method() {
    let data = vec![0x15, 0x00, 0x04, b't', b'e', b's', b't'];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x15);
    assert_eq!(value, ValueTypes::UTF8EncodedString(UTF8EncodedString("test".to_string())));
}

#[test]
fn parse_property_authentication_data() {
    let data = vec![0x16, 0x00, 0x03, 0x01, 0x02, 0x03];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x16);
    assert_eq!(value, ValueTypes::BinaryData(BinaryData(vec![0x01, 0x02, 0x03])));
}

#[test]
fn parse_property_request_problem_information() {
    let data = vec![0x17, 0x01];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x17);
    assert_eq!(value, ValueTypes::Bits(Bits(0x01)));
}

#[test]
fn parse_property_will_delay_interval() {
    let data = vec![0x18, 0x00, 0x00, 0x00, 0x01];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x18);
    assert_eq!(value, ValueTypes::FourByteInteger(FourByteInteger(0x00000001)));
}

#[test]
fn parse_property_request_response_information() {
    let data = vec![0x19, 0x01];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x19);
    assert_eq!(value, ValueTypes::Bits(Bits(0x01)));
}

#[test]
fn parse_property_response_information() {
    let data = vec![0x1A, 0x00, 0x04, b't', b'e', b's', b't'];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x1A);
    assert_eq!(value, ValueTypes::UTF8EncodedString(UTF8EncodedString("test".to_string())));
}

#[test]
fn parse_property_server_reference() {
    let data = vec![0x1C, 0x00, 0x04, b't', b'e', b's', b't'];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x1C);
    assert_eq!(value, ValueTypes::UTF8EncodedString(UTF8EncodedString("test".to_string())));
}

#[test]
fn parse_property_reason_string() {
    let data = vec![0x1F, 0x00, 0x04, b't', b'e', b's', b't'];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x1F);
    assert_eq!(value, ValueTypes::UTF8EncodedString(UTF8EncodedString("test".to_string())));
}

#[test]
fn parse_property_receive_maximum() {
    let data = vec![0x21, 0x00, 0x01];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x21);
    assert_eq!(value, ValueTypes::TwoByteInteger(TwoByteInteger(0x0001)));
}

#[test]
fn parse_property_topic_alias_maximum() {
    let data = vec![0x22, 0x00, 0x01];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x22);
    assert_eq!(value, ValueTypes::TwoByteInteger(TwoByteInteger(0x0001)));
}

#[test]
fn parse_property_topic_alias() {
    let data = vec![0x23, 0x00, 0x01];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x23);
    assert_eq!(value, ValueTypes::TwoByteInteger(TwoByteInteger(0x0001)));
}

#[test]
fn parse_property_maximum_qos() {
    let data = vec![0x24, 0x01];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x24);
    assert_eq!(value, ValueTypes::Bits(Bits(0x01)));
}

#[test]
fn parse_property_retain_available() {
    let data = vec![0x25, 0x01];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x25);
    assert_eq!(value, ValueTypes::Bits(Bits(0x01)));
}

#[test]
fn parse_property_user_property() {
    let data = vec![0x26, 0x00, 0x03, b'k', b'e', b'y', 0x00, 0x05, b'v', b'a', b'l', b'u', b'e'];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x26);
    assert_eq!(value, ValueTypes::UTF8StringPair(UTF8StringPair("key".to_string(), "value".to_string())));
}

#[test]
fn parse_property_maximum_packet_size() {
    let data = vec![0x27, 0x00, 0x00, 0x00, 0x01];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x27);
    assert_eq!(value, ValueTypes::FourByteInteger(FourByteInteger(0x00000001)));
}

#[test]
fn parse_property_wildcard_subscription_available() {
    let data = vec![0x28, 0x01];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x28);
    assert_eq!(value, ValueTypes::Bits(Bits(0x01)));
}

#[test]
fn parse_property_subscription_identifier_available() {
    let data = vec![0x29, 0x01];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x29);
    assert_eq!(value, ValueTypes::Bits(Bits(0x01)));
}

#[test]
fn parse_property_shared_subscription_available() {
    let data = vec![0x2A, 0x01];
    let result = parse_property(&data);
    assert!(result.is_ok());
    let (_, (identifier, value)) = result.unwrap();
    assert_eq!(identifier.val(), 0x2A);
    assert_eq!(value, ValueTypes::Bits(Bits(0x01)));
}

#[test]
fn parse_property_invalid_identifier() {
    let data = vec![0xFF, 0x00];
    let result = parse_property(&data);
    assert!(result.is_err());
}

#[test]
fn parse_properties_empty() {
    let data = vec![0x00];
    let result = parse_properties(&data);
    assert!(result.is_ok());
    let (_, properties) = result.unwrap();
    assert!(properties.is_empty());
}

#[test]
fn parse_properties_single_property() {
    let data = vec![0x02, 0x01, 0x12];
    let result = parse_properties(&data);
    let (_, properties) = result.unwrap();
    assert_eq!(properties.len(), 1);
    let actual = properties.get_as::<Bits>(VariableByteInteger(0x01)).unwrap();
    assert_eq!(actual, Some(&Bits(0x12)))
}

#[test]
fn parse_properties_multiple_properties() {
    let data = vec![
        0x0E, // Total length of properties
        0x01, 0x12, // PAYLOAD_FORMAT_INDICATOR
        0x02, 0x00, 0x00, 0x00, 0x01, // MESSAGE_EXPIRY_INTERVAL
        0x03, 0x00, 0x04, b't', b'e', b's', b't', // CONTENT_TYPE
    ];
    let result = parse_properties(&data);
    let (_, properties) = result.unwrap();
    assert_eq!(properties.0.len(), 3);
    assert_eq!(properties.0[&VariableByteInteger(0x01)], ValueTypes::Bits(Bits(0x12)));
    assert_eq!(properties.0[&VariableByteInteger(0x02)], ValueTypes::FourByteInteger(FourByteInteger(0x00000001)));
    assert_eq!(properties.0[&VariableByteInteger(0x03)], ValueTypes::UTF8EncodedString(UTF8EncodedString("test".to_string())));
}

#[test]
fn parse_properties_with_user_property() {
    let data = vec![
        0x0D, // Total length of properties
        0x26, 0x00, 0x03, b'k', b'e', b'y', 0x00, 0x05, b'v', b'a', b'l', b'u', b'e', // USER_PROPERTY
    ];
    let result = parse_properties(&data);
    let (_, properties) = result.unwrap();
    assert_eq!(properties.0.len(), 1);
    assert_eq!(properties.0[&VariableByteInteger(0x26)], ValueTypes::UTF8StringPair(UTF8StringPair("key".to_string(), "value".to_string())));
}
