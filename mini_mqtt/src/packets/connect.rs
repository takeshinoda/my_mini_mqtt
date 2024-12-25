use super::{
    BinaryData, Bits, ExtractValue, FixedHeader, Properties, QoS, TwoByteInteger, UTF8EncodedString,
};
use crate::errors::Error;
use std::fmt;

#[path = "connect_tests.rs"]
#[cfg(test)]
mod connect_tests;

pub struct Connect {
    pub fixed_header: FixedHeader, // 3.1.1 CONNECT Fixed Header subsection
    pub variable_header: VariableHeader, // 3.1.2 CONNECT Variable Header subsection
    pub payload: Payload,          // 3.1.3 CONNECT Payload subsection
}

impl Connect {
    pub fn new(
        fixed_header: FixedHeader,
        variable_header: VariableHeader,
        payload: Payload,
    ) -> Result<Connect, Error> {
        Ok(Connect {
            fixed_header,
            variable_header,
            payload,
        })
    }
}

// VariableHeader struct is a part of CONNECT Packet.
// 3.1.2 CONNECT Variable Header subsection
pub struct VariableHeader {
    pub protocol_name: UTF8EncodedString, // 3.1.2.1 Protocol Name subsection
    pub protocol_version: Bits,           // 3.1.2.2 Protocol Version subsection
    pub connect_flags: ConnectFlags,      // 3.1.2.3 Connect Flags subsection
    pub keep_alive: TwoByteInteger,       // seconds, 3.1.2.10 Keep Alive subsection
    pub properties: Properties,           // 3.1.2.11 CONNECT Properties subsection
}

impl VariableHeader {
    pub fn new(
        protocol_name: UTF8EncodedString,
        protocol_version: Bits,
        connect_flags: ConnectFlags,
        keep_alive: TwoByteInteger,
        properties: Properties,
    ) -> Result<VariableHeader, Error> {
        Ok(VariableHeader {
            protocol_name,
            protocol_version,
            connect_flags,
            keep_alive,
            properties,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ConnectFlags(pub Bits);

impl fmt::Display for ConnectFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ConnectFlags: username: {}, password: {}, will_retain: {}, will_qos: {}, will_flag: {}, clean_start: {}",
            self.username(),
            self.password(),
            self.will_retain(),
            self.will_qos(),
            self.will_flag(),
            self.clean_start()
        )
    }
}

impl ConnectFlags {
    pub fn new(flags: Bits) -> Result<ConnectFlags, Error> {
        Ok(ConnectFlags(flags))
    }

    pub fn username(&self) -> bool {
        self.0.val() & 0b1000_0000 != 0
    }

    pub fn password(&self) -> bool {
        self.0.val() & 0b0100_0000 != 0
    }

    pub fn will_retain(&self) -> bool {
        self.0.val() & 0b0010_0000 != 0
    }

    pub fn will_qos(&self) -> QoS {
        super::qos_from_bits(Bits((self.0.val() & 0b0001_1000) >> 3))
            .unwrap_or_else(|_| QoS::Malformed)
    }

    pub fn will_flag(&self) -> bool {
        self.0.val() & 0b0000_0100 != 0
    }

    pub fn clean_start(&self) -> bool {
        self.0.val() & 0b0000_0010 != 0
    }
}

pub struct Payload {
    pub client_id: UTF8EncodedString, // 3.1.3.1 Client Identifier (ClientID) subsection
    pub will_properties: Option<Properties>, // 3.1.3.2 Will Properties subsection
    pub will_topic: Option<UTF8EncodedString>, // 3.1.3.3 Will Topic subsection
    pub will_payload: Option<BinaryData>, // 3.1.3.4 Will Payload subsection
    pub user_name: Option<UTF8EncodedString>, // 3.1.3.5 User Name subsection
    pub password: Option<BinaryData>, // 3.1.3.6 Password subsection
}

impl Payload {
    pub fn new(
        client_id: UTF8EncodedString,
        will_properties: Option<Properties>,
        will_topic: Option<UTF8EncodedString>,
        will_payload: Option<BinaryData>,
        user_name: Option<UTF8EncodedString>,
        password: Option<BinaryData>,
    ) -> Result<Payload, Error> {
        Ok(Payload {
            client_id,
            will_properties,
            will_topic,
            will_payload,
            user_name,
            password,
        })
    }
}
