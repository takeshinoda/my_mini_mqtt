use super::*;
use crate::packets::{Bits, QoS};

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