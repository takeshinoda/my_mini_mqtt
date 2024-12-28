use super::*;
use crate::packets::{Bits, QoS, VariableByteInteger};

#[test]
fn connect_flags_username() {
    let flags = ConnectFlags::new(Bits(0b1000_0000)).unwrap();
    assert!(flags.username());
}

#[test]
fn connect_flags_no_username() {
    let flags = ConnectFlags::new(Bits(0b0000_0000)).unwrap();
    assert!(!flags.username());
}

#[test]
fn connect_flags_password() {
    let flags = ConnectFlags::new(Bits(0b0100_0000)).unwrap();
    assert!(flags.password());
}

#[test]
fn connect_flags_no_password() {
    let flags = ConnectFlags::new(Bits(0b0000_0000)).unwrap();
    assert!(!flags.password());
}

#[test]
fn connect_flags_will_retain() {
    let flags = ConnectFlags::new(Bits(0b0010_0000)).unwrap();
    assert!(flags.will_retain());
}

#[test]
fn connect_flags_no_will_retain() {
    let flags = ConnectFlags::new(Bits(0b0000_0000)).unwrap();
    assert!(!flags.will_retain());
}

#[test]
fn connect_flags_will_qos0() {
    let flags = ConnectFlags::new(Bits(0b0000_0000)).unwrap();
    assert_eq!(flags.will_qos(), QoS::AtMostOnce);
}

#[test]
fn connect_flags_will_qos1() {
    let flags = ConnectFlags::new(Bits(0b0000_1000)).unwrap();
    assert_eq!(flags.will_qos(), QoS::AtLeastOnce);
}

#[test]
fn connect_flags_will_qos2() {
    let flags = ConnectFlags::new(Bits(0b0001_0000)).unwrap();
    assert_eq!(flags.will_qos(), QoS::ExactlyOnce);
}

#[test]
fn connect_flags_will_qos_malformed() {
    let flags = ConnectFlags::new(Bits(0b0001_1000)).unwrap();
    assert_eq!(flags.will_qos(), QoS::Malformed);
}

#[test]
fn connect_flags_will_flag() {
    let flags = ConnectFlags::new(Bits(0b0000_0100)).unwrap();
    assert!(flags.will_flag());
}

#[test]
fn connect_flags_no_will_flag() {
    let flags = ConnectFlags::new(Bits(0b0000_0000)).unwrap();
    assert!(!flags.will_flag());
}

#[test]
fn connect_flags_clean_start() {
    let flags = ConnectFlags::new(Bits(0b0000_0010)).unwrap();
    assert!(flags.clean_start());
}

#[test]
fn connect_flags_no_clean_start() {
    let flags = ConnectFlags::new(Bits(0b0000_0000)).unwrap();
    assert!(!flags.clean_start());
}

fn valid_fixed_header() -> packets::FixedHeader {
    // NOTE: remaining length is 0. It is can be set to 0 for testing purposes.
    packets::FixedHeader::new(Bits(0x01), Bits(0x00), VariableByteInteger(0)).unwrap()
}

fn valid_variable_header() -> VariableHeader {
    VariableHeader::new(
        UTF8EncodedString("MQTT".to_string()),
        Bits(5),
        ConnectFlags::new(Bits(0b0000_0000)).unwrap(),
        TwoByteInteger(60),
        packets::Properties::new(),
    )
    .unwrap()
}

#[test]
fn validate_valid_connect() {
    let fixed_header = valid_fixed_header();
    let variable_header = valid_variable_header();
    let payload = Payload::new(
        UTF8EncodedString("testclient".to_string()),
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
    let connect = Connect::new(fixed_header, variable_header, payload).unwrap();
    let result = validate(&connect);
    assert!(result.is_ok());
}

#[test]
fn validate_invalid_protocol_name() {
    let fixed_header = valid_fixed_header();
    let variable_header = VariableHeader::new(
        UTF8EncodedString("MQT".to_string()), // invalid
        Bits(5),
        ConnectFlags::new(Bits(0b0000_0000)).unwrap(),
        TwoByteInteger(60),
        packets::Properties::new(),
    )
    .unwrap();
    let payload = Payload::new(
        UTF8EncodedString("testclient".to_string()),
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
    let connect = Connect::new(fixed_header, variable_header, payload).unwrap();
    let result = validate(&connect);
    assert!(result.is_err());
}

#[test]
fn validate_invalid_protocol_version() {
    let fixed_header = valid_fixed_header();
    let variable_header = VariableHeader::new(
        UTF8EncodedString("MQTT".to_string()),
        Bits(4), // unsupported version
        ConnectFlags::new(Bits(0b0000_0000)).unwrap(),
        TwoByteInteger(60),
        packets::Properties::new(),
    )
    .unwrap();
    let payload = Payload::new(
        UTF8EncodedString("testclient".to_string()),
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
    let connect = Connect::new(fixed_header, variable_header, payload).unwrap();
    let result = validate(&connect);
    assert!(result.is_err());
}

#[test]
fn validate_missing_username() {
    let fixed_header = valid_fixed_header();
    let variable_header = VariableHeader::new(
        UTF8EncodedString("MQTT".to_string()),
        Bits(5),
        ConnectFlags::new(Bits(0b1000_0000)).unwrap(), // username flag set
        TwoByteInteger(60),
        packets::Properties::new(),
    )
    .unwrap();
    let payload = Payload::new(
        UTF8EncodedString("testclient".to_string()),
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
    let connect = Connect::new(fixed_header, variable_header, payload).unwrap();
    let result = validate(&connect);
    assert!(result.is_err());
}

#[test]
fn validate_provided_username_flag_not_set() {
    let fixed_header = valid_fixed_header();
    let variable_header = valid_variable_header(); // username flag not set
    let payload = Payload::new(
        UTF8EncodedString("testclient".to_string()),
        None,
        None,
        None,
        Some(UTF8EncodedString("user".to_string())),
        None,
    )
    .unwrap();
    let connect = Connect::new(fixed_header, variable_header, payload).unwrap();
    let result = validate(&connect);
    assert!(result.is_err());
}

#[test]
fn validate_missing_password() {
    let fixed_header = valid_fixed_header();
    let variable_header = VariableHeader::new(
        UTF8EncodedString("MQTT".to_string()),
        Bits(5),
        ConnectFlags::new(Bits(0b0100_0000)).unwrap(), // password flag set
        TwoByteInteger(60),
        packets::Properties::new(),
    )
    .unwrap();
    let payload = Payload::new(
        UTF8EncodedString("testclient".to_string()),
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
    let connect = Connect::new(fixed_header, variable_header, payload).unwrap();
    let result = validate(&connect);
    assert!(result.is_err());
}

#[test]
fn validate_provided_password_flag_not_set() {
    let fixed_header = valid_fixed_header();
    let variable_header = valid_variable_header(); // password flag not set
    let payload = Payload::new(
        UTF8EncodedString("testclient".to_string()),
        None,
        None,
        None,
        None,
        Some(BinaryData(vec![0x01, 0x02, 0x03])),
    )
    .unwrap();
    let connect = Connect::new(fixed_header, variable_header, payload).unwrap();
    let result = validate(&connect);
    assert!(result.is_err());
}

#[test]
fn validate_invalid_client_id() {
    let fixed_header = valid_fixed_header();
    let variable_header = valid_variable_header();
    let payload = Payload::new(
        UTF8EncodedString("invalid_client_id!".to_string()),
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
    let connect = Connect::new(fixed_header, variable_header, payload).unwrap();
    let result = validate(&connect);
    assert!(result.is_err());
}

#[test]
fn validate_client_id_too_short() {
    let result = validate_client_id("");
    assert!(result.is_err());
}

#[test]
fn validate_client_id_too_long() {
    let result = validate_client_id("a".repeat(24).as_str());
    assert!(result.is_err());
}

#[test]
fn validate_client_id_contains_non_alphanumeric() {
    let result = validate_client_id("client_id!");
    assert!(result.is_err());
}
