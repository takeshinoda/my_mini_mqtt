use std::fmt;
use super::{
    BinaryData, Bits, ExtractValue, FixedHeader, Properties, QoS, TwoByteInteger, UTF8EncodedString,
};
use crate::errors::Error;
use crate::packets;

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

// validate the provided CONNECT Packet as the version 5.0.
pub fn validate(connect: &Connect) -> Result<(), Vec<Error>> {
    let mut errors = Vec::new();

    let fixed_header = &connect.fixed_header;
    if fixed_header.control_packet_type != Bits(packets::CONNECT) {
        errors.push(Error::MalformedPacket(
            format!("Control Packet Type is {:?}. It is not CONNECT{}",
                fixed_header.control_packet_type, packets::CONNECT).to_string()
        ));
    }
    // Ignore the flags of the Fixed Header for now...

    let variable_header = &connect.variable_header;
    if variable_header.protocol_name.val() != "MQTT" {
        errors.push(Error::MalformedPacket(
            format!("Protocol Name is not MQTT. It is {}", variable_header.protocol_name.val()).to_string()
        ));
    }

    if variable_header.protocol_version.val() != 5 {
        errors.push(Error::MalformedPacket(
            format!("Protocol Version is not 5. It is {}", variable_header.protocol_version.val()).to_string()
        ));
    }

    // Ignore around the Will Flags for now...

    if variable_header.connect_flags.username() {
        if connect.payload.user_name.is_none() {
            errors.push(Error::MalformedPacket(
                "User Name is not provided even the user name flag is 1.".to_string()
            ));
        }
    } else {
        if connect.payload.user_name.is_some() {
            errors.push(Error::MalformedPacket(
                "User Name is provided even the user name flag is 0.".to_string()
            ));
        }
    }

    if variable_header.connect_flags.password() {
        if connect.payload.password.is_none() {
            errors.push(Error::MalformedPacket(
                "Password is not provided even the password flag is 1.".to_string()
            ));
        }
    } else {
        if connect.payload.password.is_some() {
            errors.push(Error::MalformedPacket(
                "Password is provided even the password flag is 0.".to_string()
            ));
        }
    }

    // Keep Alive is not necessary to validate.

    // Ignore the following properties for now...
    // - Receive Maximum
    // - Maximum Packet Size
    // - Topic Alias Maximum
    // - Request Response Information
    // - Request Problem Information
    // - User Property
    // - Authentication Method
    // - Authentication Data

    if let Err(err) = validate_client_id(&connect.payload.client_id.val()) {
        errors.push(err);
    }

    // Ignore the Will Properties for now...

    if variable_header.connect_flags.username() {
        if connect.payload.user_name.is_none() {
            errors.push(Error::MalformedPacket(
                "User Name is not provided even the user name flag is 1.".to_string()
            ));
        }
    }

    if variable_header.connect_flags.password() {
        if connect.payload.password.is_none() {
            errors.push(Error::MalformedPacket(
                "Password is not provided even the password flag is 1.".to_string()
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

// Validate the Client ID of the CONNECT Packet.
// There is the following specification for the Client ID:
//   A Server MAY allow a Client to supply a ClientID that has a length of zero bytes, however if it does so the
//   Server MUST treat this as a special case and assign a unique ClientID to that Client [MQTT-3.1.3-6]
// But, for now, we don't support the zero-length Client ID.
// Look the 3.1.3.1 Client Identifier (ClientID) subsection for more details.
fn validate_client_id(client_id: &str) -> Result<(), Error> {
    let len = client_id.len();
    if len < 1 || len > 23 {
        return Err(Error::MalformedPacket(
            format!("Client ID length is not between 1 and 23. It is {}", client_id.len()).to_string()
        ));
    }
    // Allowed characters are: "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
    for c in client_id.chars() {
        if !c.is_ascii_alphanumeric() {
            return Err(Error::MalformedPacket(
                format!("Client ID contains non-alphanumeric character: {}", c).to_string()
            ));
        }
    }

    Ok(())
}
