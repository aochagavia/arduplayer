//! Utility functions to deal with the serial port

use std::io;
use std::process;
use std::time::Duration;

use byteorder::{LittleEndian, WriteBytesExt};
use serialport;
use serialport::prelude::*;

use util;

/// Detect available serial ports:
///
/// * If no ports are available, terminate the program
/// * If there is only one port available, choose it automatically
/// * If there are multiple ports available, prompt the user to choose
pub fn get_port_name() -> String {
    let mut ports = serialport::available_ports().unwrap();
    match ports.len() {
        0 => {
            println!("No serial ports available, exiting...");
            process::exit(0)
        }
        1 => {
            println!("Using serial port {}", ports[0].port_name);
            return ports.swap_remove(0).port_name;
        }
        _ => {
            println!("Multiple available ports:");
            for (i, p) in ports.iter().enumerate() {
                println!("{}. {}", i, p.port_name);
            }

            // Let the user choose between the available ports
            loop {
                println!("Enter the number of the port you want to use:");

                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap();

                let index = buf.trim_right().parse().unwrap();
                if index < ports.len() {
                    return ports.swap_remove(index).port_name;
                }
            }
        }
    }
}

/// Set up the serial port connection
///
/// Note: panics if there is an error
pub fn open_port(name: &str) -> Result<Box<SerialPort>, serialport::Error> {
    let settings = SerialPortSettings {
        baud_rate: BaudRate::Baud9600,
        // Fields below are set by default on the Arduino side
        data_bits: DataBits::Eight,
        parity: Parity::None,
        stop_bits: StopBits::One,
        // Some internet forum says Arduino does not use flow control
        flow_control: FlowControl::None,
        // Some random timeout... seems to work
        timeout: Duration::from_millis(5),
    };
    serialport::open_with_settings(&name, &settings)
}

/// Write a note to the serial port
pub fn write_note(port: &mut SerialPort, pin_id: u8, freq: u16) -> io::Result<()> {
    port.write_u8(pin_id)?;
    port.write_u16::<LittleEndian>(util::freq_to_delay(freq))
}
