use super::*;
use crate::packets;
use crate::packets::{
    BinaryData, Bits, FixedHeader, FourByteInteger, Packet, Properties, TwoByteInteger,
    UTF8EncodedString, UTF8StringPair, VariableByteInteger,
};

#[test]
fn encode_valid_connack_packet() {
    let mut buffer = Vec::new();
    let packet = Packet::ConnAck(packets::connack::ConnAck::default());
    let result = encode(&mut buffer, &packet);
    assert!(result.is_ok());
    assert!(!buffer.is_empty());
}

#[test]
fn encode_unsupported_packet() {
    let mut buffer = Vec::new();
    let packet = Packet::Unknown;
    let result = encode(&mut buffer, &packet);
    assert!(result.is_err());
}

#[test]
fn encode_reason_code_valid() {
    let mut buffer = Vec::new();
    let reason_code = packets::connack::SUCCESS;
    let result = encode_reason_code(&mut buffer, &reason_code);
    assert!(result.is_ok());
    assert_eq!(buffer, vec![0x00]);
}

#[test]
fn encode_value_bits() {
    let mut buffer = Vec::new();
    let value = ValueTypes::Bits(Bits(0b10101010));
    let result = encode_value(&mut buffer, &value);
    assert!(result.is_ok());
    assert_eq!(buffer, vec![0b10101010]);
}

#[test]
fn encode_value_two_byte_integer() {
    let mut buffer = Vec::new();
    let value = ValueTypes::TwoByteInteger(TwoByteInteger(0x1234));
    let result = encode_value(&mut buffer, &value);
    assert!(result.is_ok());
    assert_eq!(buffer, vec![0x12, 0x34]);
}

#[test]
fn encode_value_four_byte_integer() {
    let mut buffer = Vec::new();
    let value = ValueTypes::FourByteInteger(FourByteInteger(0x12345678));
    let result = encode_value(&mut buffer, &value);
    assert!(result.is_ok());
    assert_eq!(buffer, vec![0x12, 0x34, 0x56, 0x78]);
}

#[test]
fn encode_value_variable_byte_integer() {
    let mut buffer = Vec::new();
    let value = ValueTypes::VariableByteInteger(VariableByteInteger(300));
    let result = encode_value(&mut buffer, &value);
    assert!(result.is_ok());
    assert_eq!(buffer, vec![0xAC, 0x02]);
}

#[test]
fn encode_value_utf8_encoded_string() {
    let mut buffer = Vec::new();
    let value = ValueTypes::UTF8EncodedString(UTF8EncodedString("test".to_string()));
    let result = encode_value(&mut buffer, &value);
    assert!(result.is_ok());
    assert_eq!(buffer, vec![0x00, 0x04, b't', b'e', b's', b't']);
}

#[test]
fn encode_value_utf8_string_pair() {
    let mut buffer = Vec::new();
    let value = ValueTypes::UTF8StringPair(UTF8StringPair("key".to_string(), "value".to_string()));
    let result = encode_value(&mut buffer, &value);
    assert!(result.is_ok());
    assert_eq!(buffer, vec![0x00, 0x03, b'k', b'e', b'y', 0x00, 0x05, b'v', b'a', b'l', b'u', b'e']);
}

#[test]
fn encode_value_binary_data() {
    let mut buffer = Vec::new();
    let value = ValueTypes::BinaryData(BinaryData(vec![0x01, 0x02, 0x03]));
    let result = encode_value(&mut buffer, &value);
    assert!(result.is_ok());
    assert_eq!(buffer, vec![0x00, 0x03, 0x01, 0x02, 0x03]);
}

#[test]
fn encode_bits_valid() {
    let mut buffer = Vec::new();
    let bits = Bits(0b10101010);
    let result = encode_bits(&mut buffer, &bits);
    assert!(result.is_ok());
    assert_eq!(buffer, vec![0b10101010]);
}

#[test]
fn encode_two_byte_integer_valid() {
    let mut buffer = Vec::new();
    let integer = TwoByteInteger(0x1234);
    let result = encode_two_byte_integer(&mut buffer, &integer);
    assert!(result.is_ok());
    assert_eq!(buffer, vec![0x12, 0x34]);
}

#[test]
fn encode_four_byte_integer_valid() {
    let mut buffer = Vec::new();
    let integer = FourByteInteger(0x12345678);
    let result = encode_four_byte_integer(&mut buffer, &integer);
    assert!(result.is_ok());
    assert_eq!(buffer, vec![0x12, 0x34, 0x56, 0x78]);
}

#[test]
fn encode_variable_byte_integer_single_byte() {
    let mut buffer = Vec::new();
    let integer = packets::VariableByteInteger(127);
    let result = encode_variable_byte_integer(&mut buffer, &integer);
    assert!(result.is_ok());
    assert_eq!(buffer, vec![0x7F]);
}

#[test]
fn encode_variable_byte_integer_two_bytes() {
    let mut buffer = Vec::new();
    let integer = packets::VariableByteInteger(128);
    let result = encode_variable_byte_integer(&mut buffer, &integer);
    assert!(result.is_ok());
    assert_eq!(buffer, vec![0x80, 0x01]);
}

#[test]
fn encode_variable_byte_integer_three_bytes() {
    let mut buffer = Vec::new();
    let integer = packets::VariableByteInteger(16_383);
    let result = encode_variable_byte_integer(&mut buffer, &integer);
    assert!(result.is_ok());
    assert_eq!(buffer, vec![0xFF, 0x7F]);
}

#[test]
fn encode_variable_byte_integer_four_bytes() {
    let mut buffer = Vec::new();
    let integer = packets::VariableByteInteger(2_097_151);
    let result = encode_variable_byte_integer(&mut buffer, &integer);
    assert!(result.is_ok());
    assert_eq!(buffer, vec![0xFF, 0xFF, 0x7F]);
}

#[test]
fn encode_variable_byte_integer_max_value() {
    let mut buffer = Vec::new();
    let integer = packets::VariableByteInteger(268_435_455);
    let result = encode_variable_byte_integer(&mut buffer, &integer);
    assert!(result.is_ok());
    assert_eq!(buffer, vec![0xFF, 0xFF, 0xFF, 0x7F]);
}

#[test]
fn encode_variable_byte_integer_overflow() {
    let mut buffer = Vec::new();
    let integer = packets::VariableByteInteger(268_435_456);
    let result = encode_variable_byte_integer(&mut buffer, &integer);
    assert!(result.is_err());
}

    #[test]
fn encode_utf8_encoded_string_valid() {
    let mut buffer = Vec::new();
    let string = UTF8EncodedString("test".to_string());
    let result = encode_utf8_encoded_string(&mut buffer, &string);
    assert!(result.is_ok());
    assert_eq!(buffer, vec![0x00, 0x04, b't', b'e', b's', b't']);
}

#[test]
fn encode_utf8_string_pair_valid() {
    let mut buffer = Vec::new();
    let pair = UTF8StringPair("key".to_string(), "value".to_string());
    let result = encode_utf8_string_pair(&mut buffer, &pair);
    assert!(result.is_ok());
    assert_eq!(
        buffer,
        vec![0x00, 0x03, b'k', b'e', b'y', 0x00, 0x05, b'v', b'a', b'l', b'u', b'e']
    );
}

#[test]
fn encode_binary_data_valid() {
    let mut buffer = Vec::new();
    let data = BinaryData(vec![0x01, 0x02, 0x03]);
    let result = encode_binary_data(&mut buffer, &data);
    assert!(result.is_ok());
    assert_eq!(buffer, vec![0x00, 0x03, 0x01, 0x02, 0x03]);
}

#[test]
fn encode_fixed_header_valid() {
    let mut buffer = Vec::new();
    let header = FixedHeader::new(Bits(0x02), Bits(0x01), VariableByteInteger(0x82)).unwrap();
    let result = encode_fixed_header(&mut buffer, &header);
    assert!(result.is_ok());
    assert_eq!(buffer, vec![0x21, 0x82, 0x01]);
}

#[test]
fn encode_properties_valid() {
    let mut buffer = Vec::new();
    let mut properties = Properties::new();
    properties.insert(VariableByteInteger(1), ValueTypes::Bits(Bits(0b00000001)));
    let result = encode_properties(&mut buffer, &properties);
    assert!(result.is_ok());
    assert_eq!(buffer, vec![0x02, 0x01, 0x01]);
}

