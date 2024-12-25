use super::*;
use crate::packets::connect::{Connect, ConnectFlags, Payload};
use nom::IResult;

#[path = "connect_tests.rs"]
#[cfg(test)]
mod connect_tests;

pub fn connect_parser<'a>(
    fixed_header: FixedHeader,
) -> impl FnOnce(&'a [u8]) -> IResult<&'a [u8], Connect> {
    move |input| {
        let (input, variable_header) = parse_variable_header(input)?;
        let (input, payload) = {
            let connect_flags = &variable_header.connect_flags;
            payload_parser(connect_flags)(input)?
        };
        let connect = Connect::new(fixed_header, variable_header, payload)?;
        Ok((input, connect))
    }
}

pub fn parse_variable_header(input: &[u8]) -> IResult<&[u8], packets::connect::VariableHeader> {
    let (input, protocol_name) = parse_utf8_encoded_string(input)?; // should be "MQTT"
    let (input, protocol_version) = parse_bits(input)?; // should be 5
    let (input, connect_flags) = parse_connect_flags(input)?; //
    let (input, keep_alive) = parse_two_byte_integer(input)?;
    // CONNECT can have
    // - Session Expiry Interval
    // - Receive Maximum
    // - Maximum Packet Size
    // - Topic Alias Maximum
    // - Request Response Information
    // - Request Problem Information
    // - User Property
    // - Authentication Method
    // - Authentication Data
    let (input, properties) = parse_properties(input)?;

    let variable_header = packets::connect::VariableHeader::new(
        protocol_name,
        protocol_version,
        connect_flags,
        keep_alive,
        properties,
    )?;
    Ok((input, variable_header))
}

fn parse_connect_flags(input: &[u8]) -> IResult<&[u8], ConnectFlags> {
    let (input, flags) = parse_bits(input)?;

    let flags = ConnectFlags::new(flags)?;
    Ok((input, flags))
}

fn payload_parser<'a, 'b>(connect_flags: &'b ConnectFlags) -> impl FnOnce(&'a [u8]) -> IResult<&'a [u8], Payload> +'a {
    let connect_flags = connect_flags.clone();
    move |input|  {
        parse_payload(input, connect_flags)
    }
}

fn parse_payload(input: &[u8], connect_flags: ConnectFlags) -> IResult<&[u8], Payload> {
    // 3.1.3 CONNECT Payload subsection
    // 3.1.3.1 Client Identifier (ClientID) subsection, TODO: validation
    let (input, client_id) = parse_utf8_encoded_string(input)?;
    // 3.1.3.2 Will Properties subsection
    // If the Will Flag is set to 1, the Will Properties in the CONNECT packet MUST be present.
    // It can have the following properties:
    // - Will Delay Interval
    // - Payload Format Indicator
    // - Message Expiry Interval
    // - Content Type
    // - Response Topic
    // - Correlation Data
    // - User Property
    let (input, will_properties) =
        option_parser(parse_properties, connect_flags.will_flag())(input)?;
    // 3.1.3.3 Will Topic subsection
    let (input, will_topic) =
        option_parser(parse_utf8_encoded_string, connect_flags.will_flag())(input)?;
    // 3.1.3.4 Will Payload subsection
    let (input, will_payload) = option_parser(parse_binary_data, connect_flags.will_flag())(input)?;
    // 3.1.3.5 User Name subsection
    let (input, user_name) =
        option_parser(parse_utf8_encoded_string, connect_flags.username())(input)?;
    // 3.1.3.6 Password subsection
    let (input, password) = option_parser(parse_binary_data, connect_flags.password())(input)?;

    let payload = Payload::new(
        client_id,
        will_properties,
        will_topic,
        will_payload,
        user_name,
        password,
    )?;
    Ok((input, payload))
}
