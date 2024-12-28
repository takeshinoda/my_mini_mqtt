use std::io::Write;

use crate::codec::encoder::{encode_fixed_header, encode_properties, encode_reason_code};
use crate::packets;
use crate::errors;
use crate::packets::VariableByteInteger;

#[path = "connack_tests.rs"]
#[cfg(test)]
mod connack_tests;

pub fn encode_connack(writer : &mut dyn Write, packet : &packets::connack::ConnAck) -> Result<(), errors::Error> {
    let mut buffer = Vec::new();
    let mut vector_writer = std::io::Cursor::new(&mut buffer);

    encode_reason_code(&mut vector_writer, &packet.connect_reason_code)?;
    encode_properties(&mut vector_writer, &packet.properties)?;

   let fixed_header = packets::FixedHeader::new(
       packets::Bits(packets::CONNACK),
       packet.fixed_header.flags.clone(),
       VariableByteInteger(buffer.len() as u32),
    )?;

    encode_fixed_header(writer, &fixed_header)?;
    writer.write_all(&buffer)?;

    Ok(())
}
