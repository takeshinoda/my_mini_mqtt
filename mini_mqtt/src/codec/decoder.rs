use nom::{bytes, Err, Finish, IResult};
use std::io;
use std::io::Read; // to import the Read trait.

use crate::errors::Error;
use crate::packets;
use crate::packets::{BinaryData, Bits, ExtractValue, FixedHeader, FourByteInteger, Properties, TwoByteInteger, UTF8EncodedString, UTF8StringPair, ValueTypes, VariableByteInteger};

pub mod connect;

#[path = "decoder_tests.rs"]
#[cfg(test)]
mod decoder_tests;

pub fn decode(reader: &mut dyn Read) -> Result<packets::Packet, Error> {
    let mut buffered_reader = io::BufReader::new(reader);
    let mut buffer = [0u8; 1024];
    buffered_reader.read(&mut buffer)?;

    match parse_fixed_header(&buffer).finish() {
        Ok((_, fixed_header)) => match fixed_header.control_packet_type {
            Bits(CONNECT) => Ok(packets::Packet::Connect),
            _ => Ok(packets::Packet::Unknown),
        },
        Err(err) => Err(Error::from(err)),
    }
}

fn parse_fixed_header(input: &[u8]) -> IResult<&[u8], FixedHeader> {
    let (input, (control_packet_type, flags)) = parse_fixed_header_first_byte(input)?;
    let (input, remaining_length) = parse_variable_byte_integer(input)?;

    let fixed_header = FixedHeader::new(control_packet_type, flags, remaining_length)?;
    Ok((input, fixed_header))
}

fn parse_fixed_header_first_byte(input: &[u8]) -> IResult<&[u8], (Bits, Bits)> {
    // Input is a tuple of (input: I, bit_offset: usize)
    let (input, first_byte) = parse_bits(input)?;

    // The first 4 bits of the first byte are the control packet type.
    let control_packet_type = Bits(first_byte.val() >> 4);
    let flags = Bits(first_byte.val() & 0x0F);

    Ok((input, (control_packet_type, flags)))
}

fn parse_bits(input: &[u8]) -> IResult<&[u8], Bits> {
    let (input, bits) = bytes::complete::take(1u8)(input)?;

    Ok((input, Bits(bits[0])))
}

// The implementation of this function follows the 1.5.5 Variable Byte Integer section in the MQTT 5.0 specs.
fn parse_variable_byte_integer(input: &[u8]) -> IResult<&[u8], VariableByteInteger> {
    let mut multiplier = 1u32;
    let mut value = 0u32;
    let mut remaining_bytes = input;

    loop {
        let parsed = bytes::complete::take(1u8)(remaining_bytes)?;

        remaining_bytes = parsed.0;
        let encoded_byte = parsed.1[0];

        value += (encoded_byte & 0x7Fu8) as u32 * multiplier;

        if encoded_byte & 0x80u8 == 0 {
            break;
        }

        multiplier *= 0x80;
        if multiplier > 0x80 * 0x80 * 0x80 {
            return Err(Err::Error(nom::error::Error::new(
                remaining_bytes,
                nom::error::ErrorKind::Fail,
            )));
        }
    }

    Ok((remaining_bytes, VariableByteInteger(value)))
}

// The implementation of this function follows the 1.5.5 Variable Byte Integer section in the MQTT 5.0 specs.
// If there are invalid bytes as UTF-8 encoded string, it will be replaced with the replacement character.
fn parse_utf8_encoded_string(input: &[u8]) -> IResult<&[u8], UTF8EncodedString> {
    let (input, length) = parse_two_byte_integer(input)?;
    let (input, string) = bytes::complete::take(length.val())(input)?;

    Ok((
        input,
        UTF8EncodedString(String::from_utf8_lossy(string).to_string()),
    ))
}

fn parse_utf8_string_pair(input: &[u8]) -> IResult<&[u8], UTF8StringPair> {
    let (input, key) = parse_utf8_encoded_string(input)?;
    let (input, value) = parse_utf8_encoded_string(input)?;

    Ok((
        input,
        UTF8StringPair(key.val().to_string(), value.val().to_string()),
    ))
}

fn parse_two_byte_integer(input: &[u8]) -> IResult<&[u8], TwoByteInteger> {
    let (input, msb) = bytes::complete::take(1u8)(input)?;
    let (input, lsb) = bytes::complete::take(1u8)(input)?;

    Ok((
        input,
        TwoByteInteger(((msb[0] as u16) << 8) | lsb[0] as u16),
    ))
}

fn parse_four_byte_integer(input: &[u8]) -> IResult<&[u8], FourByteInteger> {
    let (input, bytes) = bytes::complete::take(4u8)(input)?;

    Ok((
        input,
        FourByteInteger(
            ((bytes[0] as u32) << 24)
                | ((bytes[1] as u32) << 16)
                | ((bytes[2] as u32) << 8)
                | bytes[3] as u32,
        ),
    ))
}

fn parse_binary_data(input: &[u8]) -> IResult<&[u8], BinaryData> {
    let (input, length) = parse_two_byte_integer(input)?;
    let (input, data) = bytes::complete::take(length.val() as usize)(input)?;

    Ok((input, BinaryData(data.to_vec())))
}

// value_typed_parser is a helper function to convert the parsed value to ValueTypes.
pub fn value_typed_parser<'a, F, T>(
    f: F,
) -> Box<dyn Fn(&'a [u8]) -> IResult<&'a [u8], ValueTypes> + 'a>
where
    F: Fn(&'a [u8]) -> IResult<&'a [u8], T> + 'a,
    T: Into<ValueTypes> + 'a,
{
    Box::new(move |input| f(input).map(|(i, v)| (i, v.into())))
}

// option_parser is a helper function to result the parsed value as an optional follows the provided flag.
pub fn option_parser<'a, F, T>(
    f: F,
    b: bool,
) -> Box<dyn Fn(&'a [u8]) -> IResult<&'a [u8], Option<T>> + 'a>
where
    F: Fn(&'a [u8]) -> IResult<&'a [u8], T> + 'a
{
    Box::new(move |input| {
        if b {
            f(input).map(|(i, v)| (i, Some(v)))
        } else {
            Ok((input, None))
        }
    })
}

fn parse_properties(input: &[u8]) -> IResult<&[u8], Properties> {
    let (input, properties_length) = parse_variable_byte_integer(input)?;

    let (remaining_bytes, properties_slice) =
        bytes::complete::take(properties_length.val() as usize)(input)?;

    let mut properties_bytes = properties_slice;
    let mut properties = Properties::new();
    while !properties_bytes.is_empty() {
        let (input, (identifier, value)) = parse_property(properties_bytes)?;
        properties.insert(identifier, value);
        properties_bytes = input;
    }

    Ok((remaining_bytes, properties))
}

const PAYLOAD_FORMAT_INDICATOR: VariableByteInteger = VariableByteInteger(0x01);
const MESSAGE_EXPIRY_INTERVAL: VariableByteInteger = VariableByteInteger(0x02);
const CONTENT_TYPE: VariableByteInteger = VariableByteInteger(0x03);
const RESPONSE_TOPIC: VariableByteInteger = VariableByteInteger(0x08);
const CORRELATION_DATA: VariableByteInteger = VariableByteInteger(0x09);
const SUBSCRIPTION_IDENTIFIER: VariableByteInteger = VariableByteInteger(0x0B);
const SESSION_EXPIRY_INTERVAL: VariableByteInteger = VariableByteInteger(0x11);
const ASSIGNED_CLIENT_IDENTIFIER: VariableByteInteger = VariableByteInteger(0x12);
const SERVER_KEEP_ALIVE: VariableByteInteger = VariableByteInteger(0x13);
const AUTHENTICATION_METHOD: VariableByteInteger = VariableByteInteger(0x15);
const AUTHENTICATION_DATA: VariableByteInteger = VariableByteInteger(0x16);
const REQUEST_PROBLEM_INFORMATION: VariableByteInteger = VariableByteInteger(0x17);
const WILL_DELAY_INTERVAL: VariableByteInteger = VariableByteInteger(0x18);
const REQUEST_RESPONSE_INFORMATION: VariableByteInteger = VariableByteInteger(0x19);
const RESPONSE_INFORMATION: VariableByteInteger = VariableByteInteger(0x1A);
const SERVER_REFERENCE: VariableByteInteger = VariableByteInteger(0x1C);
const REASON_STRING: VariableByteInteger = VariableByteInteger(0x1F);
const RECEIVE_MAXIMUM: VariableByteInteger = VariableByteInteger(0x21);
const TOPIC_ALIAS_MAXIMUM: VariableByteInteger = VariableByteInteger(0x22);
const TOPIC_ALIAS: VariableByteInteger = VariableByteInteger(0x23);
const MAXIMUM_QOS: VariableByteInteger = VariableByteInteger(0x24);
const RETAIN_AVAILABLE: VariableByteInteger = VariableByteInteger(0x25);
const USER_PROPERTY: VariableByteInteger = VariableByteInteger(0x26);
const MAXIMUM_PACKET_SIZE: VariableByteInteger = VariableByteInteger(0x27);
const WILDCARD_SUBSCRIPTION_AVAILABLE: VariableByteInteger = VariableByteInteger(0x28);
const SUBSCRIPTION_IDENTIFIER_AVAILABLE: VariableByteInteger = VariableByteInteger(0x29);
const SHARED_SUBSCRIPTION_AVAILABLE: VariableByteInteger = VariableByteInteger(0x2A);

// parse_property is a helper function to parse a single property from the input.
// This implementation follows the 2.2.2 Property section in the MQTT 5.0 specs.
fn parse_property(input: &[u8]) -> IResult<&[u8], (VariableByteInteger, ValueTypes)> {
    // Parse the identifier
    let (input, identifier) = parse_variable_byte_integer(input)?;

    // Select the appropriate parser using value_typed_parser
    let parse = match identifier {
        PAYLOAD_FORMAT_INDICATOR => value_typed_parser(parse_bits),
        MESSAGE_EXPIRY_INTERVAL => value_typed_parser(parse_four_byte_integer),
        CONTENT_TYPE => value_typed_parser(parse_utf8_encoded_string),
        RESPONSE_TOPIC => value_typed_parser(parse_utf8_encoded_string),
        CORRELATION_DATA => value_typed_parser(parse_binary_data),
        SUBSCRIPTION_IDENTIFIER => value_typed_parser(parse_variable_byte_integer),
        SESSION_EXPIRY_INTERVAL => value_typed_parser(parse_four_byte_integer),
        ASSIGNED_CLIENT_IDENTIFIER => value_typed_parser(parse_utf8_encoded_string),
        SERVER_KEEP_ALIVE => value_typed_parser(parse_two_byte_integer),
        AUTHENTICATION_METHOD => value_typed_parser(parse_utf8_encoded_string),
        AUTHENTICATION_DATA => value_typed_parser(parse_binary_data),
        REQUEST_PROBLEM_INFORMATION => value_typed_parser(parse_bits),
        WILL_DELAY_INTERVAL => value_typed_parser(parse_four_byte_integer),
        REQUEST_RESPONSE_INFORMATION => value_typed_parser(parse_bits),
        RESPONSE_INFORMATION => value_typed_parser(parse_utf8_encoded_string),
        SERVER_REFERENCE => value_typed_parser(parse_utf8_encoded_string),
        REASON_STRING => value_typed_parser(parse_utf8_encoded_string),
        RECEIVE_MAXIMUM => value_typed_parser(parse_two_byte_integer),
        TOPIC_ALIAS_MAXIMUM => value_typed_parser(parse_two_byte_integer),
        TOPIC_ALIAS => value_typed_parser(parse_two_byte_integer),
        MAXIMUM_QOS => value_typed_parser(parse_bits),
        RETAIN_AVAILABLE => value_typed_parser(parse_bits),
        USER_PROPERTY => value_typed_parser(parse_utf8_string_pair),
        MAXIMUM_PACKET_SIZE => value_typed_parser(parse_four_byte_integer),
        WILDCARD_SUBSCRIPTION_AVAILABLE => value_typed_parser(parse_bits),
        SUBSCRIPTION_IDENTIFIER_AVAILABLE => value_typed_parser(parse_bits),
        SHARED_SUBSCRIPTION_AVAILABLE => value_typed_parser(parse_bits),
        _ => {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            )))
        }
    };

    // Apply the selected parser
    let (input, value) = parse(input)?;

    // Return the identifier and parsed value
    Ok((input, (identifier, value)))
}

//const RESERVED: u8 = 0;
const CONNECT: u8 = 1;
/*
const CONNACK: u8 = 2;
const PUBLISH: u8 = 3;
const PUBACK: u8 = 4;
const PUBREC: u8 = 5;
const PUBREL: u8 = 6;
const PUBCOMP: u8 = 7;
const SUBSCRIBE: u8 = 8;
const SUBACK: u8 = 9;
const UNSUBSCRIBE: u8 = 10;
const UNSUBACK: u8 = 11;
const PINGREQ: u8 = 12;
const PINGRESP: u8 = 13;
const DISCONNECT: u8 = 14;
const AUTH: u8 = 15;
 */