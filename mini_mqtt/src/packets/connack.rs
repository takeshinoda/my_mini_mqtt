use crate::packets;
use crate::packets::Bits;
use crate::packets::{FixedHeader, Properties, ReasonCode, VariableByteInteger};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ConnAck {
    pub fixed_header: FixedHeader,
    pub connect_reason_code: ConnAckReasonCode, // 3.2.2.2 Connect Reason Code subsection
    pub properties: Properties,                 // 3.2.2.3 CONNACK Properties subsection
                                                // There is no payload in CONNACK packet
}

impl ConnAck {
    pub fn new(
        fixed_header: FixedHeader,
        connect_reason_code: ConnAckReasonCode,
        properties: Properties,
    ) -> ConnAck {
        ConnAck {
            fixed_header,
            connect_reason_code,
            properties,
        }
    }

    pub fn default() -> ConnAck {
        ConnAck {
            fixed_header: FixedHeader::new(Bits(packets::CONNACK), Bits(0), VariableByteInteger(2))
                .unwrap(),
            connect_reason_code: SUCCESS, // 1 byte
            properties: Properties::new(), // no properties are 1 byte
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ConnAckReasonCode(pub u8);

impl ReasonCode for ConnAckReasonCode {
    fn code(&self) -> u8 {
        self.0
    }
}

// 3.2.2.2 Connect Reason Code
// The Connection is accepted.
pub const SUCCESS: ConnAckReasonCode = ConnAckReasonCode(0x00);

// The Server does not wish to reveal the reason for the failure, or none of the other Reason Codes apply.
pub const UNSPECIFIED_ERROR: ConnAckReasonCode = ConnAckReasonCode(0x80);

// Data within the CONNECT packet could not be correctly parsed.
pub const MALFORMED_PACKET: ConnAckReasonCode = ConnAckReasonCode(0x81);

// Data in the CONNECT packet does not conform to this specification.
pub const PROTOCOL_ERROR: ConnAckReasonCode = ConnAckReasonCode(0x82);

// The CONNECT is valid but is not accepted by this Server.
pub const IMPLEMENTATION_SPECIFIC_ERROR: ConnAckReasonCode = ConnAckReasonCode(0x83);

// The Server does not support the version of the MQTT protocol requested by the Client.
pub const UNSUPPORTED_PROTOCOL_VERSION: ConnAckReasonCode = ConnAckReasonCode(0x84);

// The Client Identifier is a valid string but is not allowed by the Server.
pub const CLIENT_IDENTIFIER_NOT_VALID: ConnAckReasonCode = ConnAckReasonCode(0x85);

// The Server does not accept the User Name or Password specified by the Client.
pub const BAD_USER_NAME_OR_PASSWORD: ConnAckReasonCode = ConnAckReasonCode(0x86);

// The Client is not authorized to connect.
pub const NOT_AUTHORIZED: ConnAckReasonCode = ConnAckReasonCode(0x87);

// The MQTT Server is not available.
pub const SERVER_UNAVAILABLE: ConnAckReasonCode = ConnAckReasonCode(0x88);

// The Server is busy. Try again later.
pub const SERVER_BUSY: ConnAckReasonCode = ConnAckReasonCode(0x89);

// This Client has been banned by administrative action. Contact the server administrator.
pub const BANNED: ConnAckReasonCode = ConnAckReasonCode(0x8A);

// The authentication method is not supported or does not match the authentication method currently in use.
pub const BAD_AUTHENTICATION_METHOD: ConnAckReasonCode = ConnAckReasonCode(0x8C);

// The Will Topic Name is not malformed, but is not accepted by this Server.
pub const TOPIC_NAME_INVALID: ConnAckReasonCode = ConnAckReasonCode(0x90);

// The CONNECT packet exceeded the maximum permissible size.
pub const PACKET_TOO_LARGE: ConnAckReasonCode = ConnAckReasonCode(0x95);

// An implementation or administrative imposed limit has been exceeded.
pub const QUOTA_EXCEEDED: ConnAckReasonCode = ConnAckReasonCode(0x97);

// The Will Payload does not match the specified Payload Format Indicator.
pub const PAYLOAD_FORMAT_INVALID: ConnAckReasonCode = ConnAckReasonCode(0x99);

// The Server does not support retained messages, and Will Retain was set to 1.
pub const RETAIN_NOT_SUPPORTED: ConnAckReasonCode = ConnAckReasonCode(0x9A);

// The Server does not support the QoS set in Will QoS.
pub const QOS_NOT_SUPPORTED: ConnAckReasonCode = ConnAckReasonCode(0x9B);

// The Client should temporarily use another server.
pub const USE_ANOTHER_SERVER: ConnAckReasonCode = ConnAckReasonCode(0x9C);

// The Client should permanently use another server.
pub const SERVER_MOVED: ConnAckReasonCode = ConnAckReasonCode(0x9D);

// The connection rate limit has been exceeded.
pub const CONNECTION_RATE_EXCEEDED: ConnAckReasonCode = ConnAckReasonCode(0x9F);
