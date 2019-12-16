#[allow(dead_code)]
use serialport::{DataBits, FlowControl, Parity, SerialPortSettings, StopBits};
use std::io::Error;
use std::time::Duration;

pub struct SerialContructor {
    baud_rate: u32,
    data_bits: Option<DataBits>,
    flow_control: Option<FlowControl>,
    parity: Option<Parity>,
    stop_bits: Option<StopBits>,
    timeout: Option<Duration>,
}

impl SerialContructor {
    pub fn new(baud_rate: u32) -> Self {
        Self {
            baud_rate: baud_rate,
            data_bits: None,
            flow_control: None,
            parity: None,
            stop_bits: None,
            timeout: None,
        }
    }

    pub fn with_data_bits(&mut self, data_bits: DataBits) -> &mut Self {
        self.data_bits = Some(data_bits);
        self
    }

    pub fn with_flow(&mut self, flow: FlowControl) -> &mut Self {
        self.flow_control = Some(flow);
        self
    }

    pub fn with_parity(&mut self, parity: Parity) -> &mut Self {
        self.parity = Some(parity);
        self
    }

    pub fn with_stop_bits(&mut self, stop_bits: StopBits) -> &mut Self {
        self.stop_bits = Some(stop_bits);
        self
    }

    pub fn with_timeout(&mut self, timeout: Duration) -> &mut Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn gen(self) -> Result<SerialPortSettings, Error> {
        let settings = SerialPortSettings {
            baud_rate: self.baud_rate,
            data_bits: {
                match self.data_bits {
                    Some(v) => v,
                    None => DataBits::Eight,
                }
            },
            flow_control: {
                match self.flow_control {
                    Some(v) => v,
                    None => FlowControl::None,
                }
            },
            parity: {
                match self.parity {
                    Some(v) => v,
                    None => Parity::None,
                }
            },
            stop_bits: {
                match self.stop_bits {
                    Some(v) => v,
                    None => StopBits::One,
                }
            },
            timeout: {
                match self.timeout {
                    Some(v) => v,
                    None => Duration::from_millis(1),
                }
            },
        };

        Ok(settings)
    }
}
