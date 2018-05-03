use std::io;

use byteorder::{LittleEndian, WriteBytesExt};
use serialport::SerialPort;

use util;

#[derive(Debug)]
pub struct PinId(u8);

#[derive(Debug)]
pub struct Command {
    pin_id: PinId,
    frequency: u16
}

impl Command {
    pub fn new(pin_id: u8, frequency: u16) -> Command {
        Command {
            pin_id: PinId(pin_id),
            frequency
        }
    }

    pub fn write(&self, port: &mut SerialPort) -> io::Result<()> {
        port.write_u8(self.pin_id.0)?;
        port.write_u16::<LittleEndian>(util::freq_to_delay(self.frequency))?;

        Ok(())
    }
}
