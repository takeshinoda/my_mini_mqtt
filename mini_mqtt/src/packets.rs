use std::collections::HashMap;
use std::fmt;

use crate::errors::Error;

pub mod connect;

#[path = "packets_tests.rs"]
#[cfg(test)]
mod packets_tests;

pub enum Packet {
    Unknown,
    Reserved,
    //Connect(connect::Connect),
    Connect,
    ConnAck,
    Publish,
    PubAck,
    PubRec,
    PubRel,
    PubComp,
    Subscribe,
    SubAck,
    Unsubscribe,
    UnSuback,
    PingReq,
    PingResp,
    Disconnect,
    Auth,
}

// Control Packet Type of the Fixed Header.
//pub const RESERVED: u8 = 0;
pub const CONNECT: u8 = 1;
/*
pub const CONNACK: u8 = 2;
pub const PUBLISH: u8 = 3;
pub const PUBACK: u8 = 4;
pub const PUBREC: u8 = 5;
pub const PUBREL: u8 = 6;
pub const PUBCOMP: u8 = 7;
pub const SUBSCRIBE: u8 = 8;
pub const SUBACK: u8 = 9;
pub const UNSUBSCRIBE: u8 = 10;
pub const UNSUBACK: u8 = 11;
const PINGREQ: u8 = 12;
const PINGRESP: u8 = 13;
const DISCONNECT: u8 = 14;
const AUTH: u8 = 15;
 */

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Bits(pub u8); // 1.5.1 Bits subsection
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TwoByteInteger(pub u16); // 1.5.2 Two Byte Integer subsection
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FourByteInteger(pub u32); // 1.5.3 Four Byte Integer subsection
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UTF8EncodedString(pub String); // 1.5.4 UTF-8 Encoded String subsection
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct VariableByteInteger(pub u32); // 1.5.5 Variable Byte Integer subsection
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BinaryData(pub Vec<u8>); // 1.5.6 Binary Data subsection
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UTF8StringPair(pub String, pub String); // 1.5.7 UTF-8 String Pair subsection

pub trait ExtractValue<'a, T> {
    fn val(&'a self) -> T;
}

// I need to consider the following implementations' structures...
impl ExtractValue<'_, u8> for Bits {
    fn val(&self) -> u8 {
        self.0
    }
}

impl ExtractValue<'_, u16> for TwoByteInteger {
    fn val(&self) -> u16 {
        self.0
    }
}

impl ExtractValue<'_, u32> for FourByteInteger {
    fn val(&self) -> u32 {
        self.0
    }
}

impl ExtractValue<'_, u32> for VariableByteInteger {
    fn val(&self) -> u32 {
        self.0
    }
}

impl<'a> ExtractValue<'a, &'a str> for UTF8EncodedString {
    fn val(&'a self) -> &'a str {
        self.0.as_str()
    }
}

impl<'a> ExtractValue<'a, &'a Vec<u8>> for BinaryData {
    fn val(&'a self) -> &'a Vec<u8> {
        &self.0
    }
}

impl<'a> ExtractValue<'a, (&'a str, &'a str)> for UTF8StringPair {
    fn val(&'a self) -> (&'a str, &'a str) {
        (self.0.as_str(), self.1.as_str())
    }
}

#[derive(Debug, PartialEq)]
pub enum ValueTypes {
    Bits(Bits),
    TwoByteInteger(TwoByteInteger),
    FourByteInteger(FourByteInteger),
    UTF8EncodedString(UTF8EncodedString),
    VariableByteInteger(VariableByteInteger),
    BinaryData(BinaryData),
    UTF8StringPair(UTF8StringPair),
}

pub trait FromValueTypesRef<'a>: Sized {
    fn from_value_types_ref(value: &'a ValueTypes) -> Option<&'a Self>;
    fn type_name() -> &'static str;
}

impl<'a> FromValueTypesRef<'a> for Bits {
    fn from_value_types_ref(value: &'a ValueTypes) -> Option<&'a Self> {
        match value {
            ValueTypes::Bits(b) => Some(b),
            _ => None,
        }
    }

    fn type_name() -> &'static str {
        "Bits"
    }
}

impl<'a> FromValueTypesRef<'a> for TwoByteInteger {
    fn from_value_types_ref(value: &'a ValueTypes) -> Option<&'a Self> {
        match value {
            ValueTypes::TwoByteInteger(t) => Some(t),
            _ => None,
        }
    }

    fn type_name() -> &'static str {
        "TwoByteInteger"
    }
}

impl<'a> FromValueTypesRef<'a> for FourByteInteger {
    fn from_value_types_ref(value: &'a ValueTypes) -> Option<&'a Self> {
        match value {
            ValueTypes::FourByteInteger(f) => Some(f),
            _ => None,
        }
    }

    fn type_name() -> &'static str {
        "FourByteInteger"
    }
}

impl<'a> FromValueTypesRef<'a> for UTF8EncodedString {
    fn from_value_types_ref(value: &'a ValueTypes) -> Option<&'a Self> {
        match value {
            ValueTypes::UTF8EncodedString(u) => Some(u),
            _ => None,
        }
    }

    fn type_name() -> &'static str {
        "UTF8EncodedString"
    }
}

impl<'a> FromValueTypesRef<'a> for VariableByteInteger {
    fn from_value_types_ref(value: &'a ValueTypes) -> Option<&'a Self> {
        match value {
            ValueTypes::VariableByteInteger(v) => Some(v),
            _ => None,
        }
    }

    fn type_name() -> &'static str {
        "VariableByteInteger"
    }
}

impl<'a> FromValueTypesRef<'a> for BinaryData {
    fn from_value_types_ref(value: &'a ValueTypes) -> Option<&'a Self> {
        match value {
            ValueTypes::BinaryData(b) => Some(b),
            _ => None,
        }
    }

    fn type_name() -> &'static str {
        "BinaryData"
    }
}

impl<'a> FromValueTypesRef<'a> for UTF8StringPair {
    fn from_value_types_ref(value: &'a ValueTypes) -> Option<&'a Self> {
        match value {
            ValueTypes::UTF8StringPair(u) => Some(u),
            _ => None,
        }
    }

    fn type_name() -> &'static str {
        "UTF8StringPair"
    }
}

impl From<Bits> for ValueTypes {
    fn from(bits: Bits) -> Self {
        ValueTypes::Bits(bits)
    }
}

impl From<TwoByteInteger> for ValueTypes {
    fn from(two_byte_integer: TwoByteInteger) -> Self {
        ValueTypes::TwoByteInteger(two_byte_integer)
    }
}

impl From<FourByteInteger> for ValueTypes {
    fn from(four_byte_integer: FourByteInteger) -> Self {
        ValueTypes::FourByteInteger(four_byte_integer)
    }
}

impl From<UTF8EncodedString> for ValueTypes {
    fn from(utf8_encoded_string: UTF8EncodedString) -> Self {
        ValueTypes::UTF8EncodedString(utf8_encoded_string)
    }
}

impl From<VariableByteInteger> for ValueTypes {
    fn from(variable_byte_integer: VariableByteInteger) -> Self {
        ValueTypes::VariableByteInteger(variable_byte_integer)
    }
}

impl From<BinaryData> for ValueTypes {
    fn from(binary_data: BinaryData) -> Self {
        ValueTypes::BinaryData(binary_data)
    }
}

impl From<UTF8StringPair> for ValueTypes {
    fn from(utf8_string_pair: UTF8StringPair) -> Self {
        ValueTypes::UTF8StringPair(utf8_string_pair)
    }
}

impl<'a, T> ExtractValue<'a, T> for ValueTypes
where
    Bits: ExtractValue<'a, T>,
    TwoByteInteger: ExtractValue<'a, T>,
    FourByteInteger: ExtractValue<'a, T>,
    UTF8EncodedString: ExtractValue<'a, T>,
    VariableByteInteger: ExtractValue<'a, T>,
    BinaryData: ExtractValue<'a, T>,
    UTF8StringPair: ExtractValue<'a, T>,
{
    fn val(&'a self) -> T {
        match self {
            ValueTypes::Bits(value) => value.val(),
            ValueTypes::TwoByteInteger(value) => value.val(),
            ValueTypes::FourByteInteger(value) => value.val(),
            ValueTypes::UTF8EncodedString(value) => value.val(),
            ValueTypes::VariableByteInteger(value) => value.val(),
            ValueTypes::BinaryData(value) => value.val(),
            ValueTypes::UTF8StringPair(value) => value.val(),
        }
    }
}

impl fmt::Display for ValueTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueTypes::Bits(b) => write!(f, "Bits({})", b.val()),
            ValueTypes::TwoByteInteger(t) => write!(f, "TwoByteInteger({})", t.val()),
            ValueTypes::FourByteInteger(four) => write!(f, "FourByteInteger({})", four.val()),
            ValueTypes::UTF8EncodedString(u) => write!(f, "UTF8EncodedString({})", u.val()),
            ValueTypes::VariableByteInteger(v) => write!(f, "VariableByteInteger({})", v.val()),
            ValueTypes::BinaryData(b) => write!(f, "BinaryData({:?})", b.val()),
            ValueTypes::UTF8StringPair(u) => write!(f, "UTF8StringPair({:?})", u.val()),
        }
    }
}

#[derive(Debug)]
pub struct FixedHeader {
    pub control_packet_type: Bits,
    pub flags: Bits,
    pub remaining_length: VariableByteInteger, // necessary...?
}

impl FixedHeader {
    pub fn new(
        control_packet_type: Bits,
        flags: Bits,
        remaining_length: VariableByteInteger,
    ) -> Result<FixedHeader, Error> {
        Ok(FixedHeader {
            control_packet_type,
            flags,
            remaining_length,
        })
    }

    // take_flag results a bit by the provided bitnum's bit.
    pub fn take_flag(&self, bitnum: u8) -> u8 {
        (self.flags.val() & (1 << bitnum)) >> bitnum
    }
}

impl fmt::Display for FixedHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FixedHeader {{ control_packet_type: {}, flags: {} }}",
            self.control_packet_type.val(),
            self.flags.val()
        )
    }
}

// PacketIdentity is a 16-bit unsigned integer that identifies a packet.
// It is used in the MQTT 5.0 protocol to identify packets(2.2.1).
#[derive(Debug, PartialEq, Eq)]
pub struct PacketIdentity(TwoByteInteger);

pub trait PacketIdentifier {
    fn packet_identity(&self) -> &PacketIdentity;
}

pub struct Properties(pub HashMap<VariableByteInteger, ValueTypes>); // 2.2.2 Property Length subsection

impl Properties {
    pub fn new() -> Self {
        Properties(HashMap::new())
    }

    pub fn insert(&mut self, key: VariableByteInteger, value: ValueTypes) {
        self.0.insert(key, value);
    }

    pub fn get_as<'a, T>(&'a self, key: VariableByteInteger) -> Result<Option<&'a T>, Error>
    where
        T: FromValueTypesRef<'a>,
    {
        let val = self.0.get(&key);
        match val {
            Some(value) => {
                if let Some(typed) = T::from_value_types_ref(value) {
                    Ok(Some(typed))
                } else {
                    Err(Error::Common(format!(
                        "The provided identifier is not a {:?} type. The stored value's type: {:?}",
                        T::type_name(),
                        value
                    )))
                }
            }
            None => Ok(None),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum QoS {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce,
    Malformed,
}

impl fmt::Display for QoS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QoS::AtMostOnce => write!(f, "AtMostOnce(0)"),
            QoS::AtLeastOnce => write!(f, "AtLeastOnce(1)"),
            QoS::ExactlyOnce => write!(f, "ExactlyOnce(2)"),
            QoS::Malformed => write!(f, "Malformed"),
        }
    }
}

pub fn qos_from_bits(qos: Bits) -> Result<QoS, Error> {
    let val = qos.val();
    match val {
        0 => Ok(QoS::AtMostOnce),
        1 => Ok(QoS::AtLeastOnce),
        2 => Ok(QoS::ExactlyOnce),
        _ => Err(Error::MalformedPacket(format!("Invalid QoS value {}", val))),
    }
}
