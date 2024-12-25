
use crate::packets::*;

#[test]
fn test_get_bits() {
    let mut properties = Properties::new();
    properties.insert(VariableByteInteger(0x01), ValueTypes::Bits(Bits(0x12)));
    let result = properties.get_as::<Bits>(VariableByteInteger(0x01)).unwrap();
    assert_eq!(result, Some(&Bits(0x12)));
}

#[test]
fn test_get_two_byte_integer() {
    let mut properties = Properties::new();
    properties.insert(VariableByteInteger(0x02), ValueTypes::TwoByteInteger(TwoByteInteger(0x1234)));
    let result = properties.get_as::<TwoByteInteger>(VariableByteInteger(0x02)).unwrap();
    assert_eq!(result, Some(&TwoByteInteger(0x1234)));
}

#[test]
fn test_get_four_byte_integer() {
    let mut properties = Properties::new();
    properties.insert(VariableByteInteger(0x03), ValueTypes::FourByteInteger(FourByteInteger(0x12345678)));
    let result = properties.get_as::<FourByteInteger>(VariableByteInteger(0x03)).unwrap();
    assert_eq!(result, Some(&FourByteInteger(0x12345678)));
}

#[test]
fn test_get_utf8_encoded_string() {
    let mut properties = Properties::new();
    properties.insert(VariableByteInteger(0x04), ValueTypes::UTF8EncodedString(UTF8EncodedString("test".to_string())));
    let result = properties.get_as::<UTF8EncodedString>(VariableByteInteger(0x04)).unwrap();
    assert_eq!(result, Some(&UTF8EncodedString("test".to_string())));
}

#[test]
fn test_get_binary_data() {
    let mut properties = Properties::new();
    properties.insert(VariableByteInteger(0x05), ValueTypes::BinaryData(BinaryData(vec![0x01, 0x02, 0x03])));
    let result = properties.get_as::<BinaryData>(VariableByteInteger(0x05)).unwrap();
    assert_eq!(result, Some(&BinaryData(vec![0x01, 0x02, 0x03])));
}

#[test]
fn test_get_utf8_string_pair() {
    let mut properties = Properties::new();
    properties.insert(VariableByteInteger(0x06), ValueTypes::UTF8StringPair(UTF8StringPair("key".to_string(), "value".to_string())));
    let result = properties.get_as::<UTF8StringPair>(VariableByteInteger(0x06)).unwrap();
    assert_eq!(result, Some(&UTF8StringPair("key".to_string(), "value".to_string())));
}

#[test]
fn test_get_non_existent_key() {
    let properties = Properties::new();
    let result: Option<&Bits> = properties.get_as::<Bits>(VariableByteInteger(0x01)).unwrap();
    assert_eq!(result, None);
}

#[test]
fn test_get_wrong_type() {
    let mut properties = Properties::new();
    properties.insert(VariableByteInteger(0x01), ValueTypes::Bits(Bits(0x12)));
    let result = properties.get_as::<TwoByteInteger>(VariableByteInteger(0x01));
    assert!(result.is_err());
}
