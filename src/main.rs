mod builder;

use crate::builder::SerialContructor;
use serialport::{DataBits, FlowControl, Parity, SerialPortSettings, StopBits};
use std::io::Error;
use std::io::Write;
use std::time::Duration;

pub fn handle_serial(port: String, settings: SerialPortSettings) -> Result<(), std::io::Error> {
    println!("{:?}", &settings);
    let settings = SerialPortSettings::default();
    // println!("default: {:?}", d);
    match serialport::open_with_settings(&port, &settings) {
        Ok(mut p) => {
            let mut buf: Vec<u8> = Vec::with_capacity(50);
            println!(
                "Recieving data on {} at {} baud",
                &port, &settings.baud_rate
            );

            loop {
                match p.read(buf.as_mut_slice()) {
                    Ok(t) => std::io::stdout().write_all(&buf[..t]).unwrap(),
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                    Err(e) => {
                        println!("{:?}", e);
                        return Err(e);
                    }
                }
            }
        }
        Err(e) => {
            //
            println!("Defintely an error: {}", e);
            Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                format!("Serial port not opened: {}", e),
            ))
        }
    }
}

fn main() {
    let baud_rate = 115200;
    let port = "/dev/ttyS5".to_string();
    let mut builder = SerialContructor::new(baud_rate);

    builder
        .with_data_bits(DataBits::Eight)
        .with_flow(FlowControl::None)
        .with_parity(Parity::None)
        .with_stop_bits(StopBits::One);

    let res = builder
        .gen()
        .map(move |settings| {
            //
            if let Err(err) = handle_serial(port, settings) {
                //
                println!("Something went wrong with the serial read: {}", err);
            }
        })
        .map_err(|e| println!("Error: {}", e));
    println!("{:?}", res);
}
