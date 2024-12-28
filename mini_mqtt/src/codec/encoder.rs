use crate::errors;
use crate::packets;
use crate::packets::{ExtractValue, ValueTypes};
use std::io::Write;

pub mod connack;

#[path = "encoder_tests.rs"]
#[cfg(test)]
mod encoder_tests;

pub fn encode(writer: &mut dyn Write, packet: &packets::Packet) -> Result<(), errors::Error> {
    match packet {
        packets::Packet::ConnAck(packet) => {
            connack::encode_connack(writer, packet)?;
        }
        _ => {
            return Err(errors::Error::Common("Not implemented yet".to_string()));
        }
    }

    Ok(())
}

pub fn encode_reason_code(
    writer: &mut dyn Write,
    reason_code: &dyn packets::ReasonCode,
) -> Result<(), errors::Error> {
    let code = reason_code.code();
    writer.write_all(&[code])?;
    Ok(())
}

pub fn encode_value(writer: &mut dyn Write, value: &ValueTypes) -> Result<(), errors::Error> {
    match value {
        ValueTypes::Bits(val) => encode_bits(writer, &val),
        ValueTypes::TwoByteInteger(val) => encode_two_byte_integer(writer, &val),
        ValueTypes::FourByteInteger(val) => encode_four_byte_integer(writer, &val),
        ValueTypes::VariableByteInteger(val) => encode_variable_byte_integer(writer, &val),
        ValueTypes::UTF8EncodedString(val) => encode_utf8_encoded_string(writer, &val),
        ValueTypes::UTF8StringPair(val) => encode_utf8_string_pair(writer, &val),
        ValueTypes::BinaryData(val) => encode_binary_data(writer, &val),
    }
}

pub fn encode_bits(writer: &mut dyn Write, bits: &packets::Bits) -> Result<(), errors::Error> {
    let bits = bits.val();
    writer.write_all(&[bits])?;
    Ok(())
}

pub fn encode_two_byte_integer(
    writer: &mut dyn Write,
    integer: &packets::TwoByteInteger,
) -> Result<(), errors::Error> {
    let integer = integer.val();
    writer.write_all(&[(integer >> 8) as u8, integer as u8])?;
    Ok(())
}

pub fn encode_four_byte_integer(
    writer: &mut dyn Write,
    integer: &packets::FourByteInteger,
) -> Result<(), errors::Error> {
    let integer = integer.val();
    writer.write_all(&[
        (integer >> 24) as u8,
        (integer >> 16) as u8,
        (integer >> 8) as u8,
        integer as u8,
    ])?;
    Ok(())
}

pub fn encode_variable_byte_integer(
    writer: &mut dyn Write,
    integer: &packets::VariableByteInteger,
) -> Result<(), errors::Error> {
    let mut value = integer.val();
    let mut buffer = Vec::new();

   if value > 268_435_455 {
        return Err(errors::Error::Common(format!(
            "Variable Byte Integer is too big: {}, max is 268_435_455",
            value
        )));
    }

    // See 1.5.5 Variable Byte Integer subsection
    loop {
        let mut byte = (value % 0x80) as u8;
        value /= 0x80;
        if value > 0 {
            byte |= 0x80;
        }
        buffer.push(byte);
        if value == 0 {
            break;
        }
    }

    writer.write_all(&buffer)?;
    Ok(())
}

pub fn encode_utf8_encoded_string(
    writer: &mut dyn Write,
    string: &packets::UTF8EncodedString,
) -> Result<(), errors::Error> {
    encode_string(writer, string.val())?;
    Ok(())
}

pub fn encode_utf8_string_pair(
    writer: &mut dyn Write,
    pair: &packets::UTF8StringPair,
) -> Result<(), errors::Error> {
    let (key, value) = pair.val();
    encode_string(writer, key)?;
    encode_string(writer, value)?;
    Ok(())
}

pub fn encode_string(writer: &mut dyn Write, string: &str) -> Result<(), errors::Error> {
    let length = string.len();

    if length > 0xFFFF {
        return Err(errors::Error::Common(format!(
            "String is too long: {}",
            length
        )));
    }

    encode_two_byte_integer(writer, &packets::TwoByteInteger(length as u16))?;
    writer.write_all(string.as_bytes())?;
    Ok(())
}

pub fn encode_binary_data(
    writer: &mut dyn Write,
    data: &packets::BinaryData,
) -> Result<(), errors::Error> {
    let data = data.val();
    let length = data.len();

    if length > 0xFFFF {
        return Err(errors::Error::Common(format!(
            "Binary is too long: {}",
            length
        )));
    }

    encode_two_byte_integer(writer, &packets::TwoByteInteger(length as u16))?;
    writer.write_all(data)?;
    Ok(())
}

pub fn encode_properties(
    writer: &mut dyn Write,
    properties: &packets::Properties,
) -> Result<(), errors::Error> {
    let mut buffer = Vec::new();
    let mut vector_writer = std::io::Cursor::new(&mut buffer);

    for (key, value) in properties.iter() {
        encode_variable_byte_integer(&mut vector_writer, key)?;
        encode_value(&mut vector_writer, value)?;
    }

    let length = buffer.len();
    if length > u32::MAX as usize {
        return Err(errors::Error::Common(format!(
            "Properties are too long: {}",
            length
        )));
    }

    encode_variable_byte_integer(writer, &packets::VariableByteInteger(length as u32))?;
    writer.write_all(&buffer)?;

    Ok(())
}
pub fn encode_fixed_header(
    writer: &mut dyn Write,
    header: &packets::FixedHeader,
) -> Result<(), errors::Error> {
    let byte1 = header.control_packet_type.val() << 4 | header.flags.val() & 0x0F;
    writer.write_all(&[byte1])?;
    encode_variable_byte_integer(writer, &header.remaining_length)?;
    Ok(())
}
