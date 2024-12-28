
use crate::codec::encoder::connack::*; // The test targets

use crate::packets;
use crate::packets::{Bits, VariableByteInteger, UTF8EncodedString};

#[test]
fn encode_connack_success() {
    let mut buffer = Vec::new();

    let mut properties = packets::Properties::new();
    properties.insert(
        packets::CONTENT_TYPE,
        packets::ValueTypes::UTF8EncodedString(UTF8EncodedString("test".to_string())));

    let packet = packets::connack::ConnAck {
        fixed_header: packets::FixedHeader::new(Bits(0b00100000), Bits(0), VariableByteInteger(0)).unwrap(),
        connect_reason_code: packets::connack::SUCCESS,
        properties,
    };

    let expected_data = vec![
        0x00u8, // Reason code
        0x07, // Properties length (Variable Byte Integer)
        0x03,  // Content Type
        0x00, 0x04, // UTF-8 Encoded String length
        b't', b'e', b's', b't', // UTF-8 Encoded String
    ];
    let fixed_header_data= vec![
        0b0010_0000u8, // Fixed header
        expected_data.len() as u8, // Remaining length (Variable Byte Integer)
    ];
    let expected_data = [ fixed_header_data, expected_data ].concat();

    let result = encode_connack(&mut buffer, &packet);
    assert!(result.is_ok());
    assert_eq!(buffer, expected_data);
}

#[test]
fn encode_connack_with_no_properties() {
    let mut buffer = Vec::new();
    let packet = packets::connack::ConnAck {
        fixed_header: packets::FixedHeader::new(Bits(0b00100000), Bits(0), VariableByteInteger(0)).unwrap(),
        connect_reason_code: packets::connack::SUCCESS,
        properties : packets::Properties::new(),
    };

    let expected = vec![
        0b0010_0000u8, // Fixed header
        0x02u8, // Remaining length (Variable Byte Integer)
        0x00u8, // Reason code
        0x00, // Properties length (Variable Byte Integer)
    ];

    let result = encode_connack(&mut buffer, &packet);
    assert!(result.is_ok());
    assert_eq!(buffer, expected);
}
