use crate::anc::AncMode;
use crate::connect::connect;
use crate::Nothing;
use bluer::rfcomm::Stream;
use bluer::{Error, ErrorKind};
use std::future::Future;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use tokio::time::sleep;

const EAR2_ADDRESS: [u8; 3] = [0x2C, 0xBE, 0xEB];
const EAR2_CHANNEL: u8 = 15;
const EAR2_FIRMWARE: [u8; 10] = [0x55, 0x60, 0x01, 0x42, 0xc0, 0x00, 0x00, 0x03, 0xe0, 0xd1];
const EAR2_SERIAL: [u8; 10] = [0x55, 0x60, 0x01, 0x06, 0xc0, 0x00, 0x00, 0x05, 0x90, 0xdc];
const RETRY: u64 = 3;

const EAR2_LOW_LAG_ON: [u8; 12] = [
    0x55, 0x60, 0x01, 0x40, 0xf0, 0x02, 0x00, 0x27, 0x01, 0x00, 0x97, 0xf7,
];

const EAR2_LOW_LAG_OFF: [u8; 12] = [
    0x55, 0x60, 0x01, 0x40, 0xf0, 0x02, 0x00, 0x28, 0x02, 0x00, 0xa7, 0x04,
];

const EAR2_ANC: [u8; 13] = [
    0x55, 0x60, 0x01, 0x0f, 0xf0, 0x03, 0x00, 0xcd, 0x01, 0x00, 0x00, 0xc4, 0x47,
];

const EAR2_IN_EAR_DETECTION_ON: [u8; 13] = [
    0x55, 0x60, 0x01, 0x04, 0xf0, 0x03, 0x00, 0x26, 0x01, 0x01, 0x01, 0x73, 0x10,
];

const EAR2_IN_EAR_DETECTION_OFF: [u8; 13] = [
    0x55, 0x60, 0x01, 0x04, 0xf0, 0x03, 0x00, 0x25, 0x01, 0x01, 0x00, 0xb2, 0x94,
];

pub type Ear2State = Mutex<Ear2>;

pub struct Ear2 {
    pub address: Option<String>,
    pub firmware_version: Option<String>,
    pub serial_number: Option<String>,
    stream: Option<Stream>,
}

impl Nothing for Ear2 {
    async fn get_address(&self) -> Option<String> {
        self.address.clone()
    }
    async fn get_firmware_version(&self) -> Option<String> {
        self.firmware_version.clone()
    }
    async fn get_serial_number(&self) -> Option<String> {
        self.serial_number.clone()
    }

    fn set_anc_mode(&mut self, mode: AncMode) -> impl Future<Output = Result<(), Error>> + Send {
        self.set_anc(mode)
    }

    fn set_low_latency_mode(
        &mut self,
        mode: bool,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        self.set_low_latency(mode)
    }

    fn set_in_ear_detection_mode(
        &mut self,
        mode: bool,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        self.set_in_ear_detection(mode)
    }

    fn try_connect(&mut self) -> impl Future<Output = Result<(), Error>> + Send {
        self.try_connect()
    }
}

impl Ear2 {
    pub async fn new() -> bluer::Result<Self> {
        let mut ear_2 = Self {
            address: None,
            firmware_version: None,
            serial_number: None,
            stream: None,
        };

        // try to connect to the device
        ear_2.try_connect().await?;

        Ok(ear_2)
    }

    pub async fn try_connect(&mut self) -> bluer::Result<()> {
        let stream = Self::fetch_stream().await;
        if let Some(mut stream) = stream {
            // get firmware version
            stream.write_all(&EAR2_FIRMWARE).await?;
            let mut buf = [0_u8; 8];
            stream.read_exact(&mut buf).await?;
            let version_str_len: usize = buf[5].try_into().unwrap();
            let mut buf = vec![0_u8; version_str_len + 2];
            stream.read_exact(&mut buf).await?;
            let version = String::from_utf8_lossy(&buf[..version_str_len]);

            // get serial number
            stream.write_all(&EAR2_SERIAL).await?;
            let mut buf = [0_u8; 64 + 64 + 18];
            stream.read_exact(&mut buf).await?;
            let serial = String::from_utf8_lossy(&buf[37..53]);

            self.address = Some(stream.peer_addr()?.addr.to_string());
            self.firmware_version = Some(version.to_string());
            self.serial_number = Some(serial.to_string());
            self.stream = Some(stream);
        }

        Ok(())
    }

    pub async fn fetch_stream() -> Option<Stream> {
        let mut stream = connect(EAR2_ADDRESS, EAR2_CHANNEL).await;
        for i in 1..=RETRY {
            sleep(std::time::Duration::from_millis(i * 690)).await;
            match stream {
                Ok(s) => {
                    stream = Ok(s);
                    break;
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    stream = connect(EAR2_ADDRESS, EAR2_CHANNEL).await;
                }
            }
        }

        stream.ok()
    }

    pub async fn set_anc(&mut self, mode: AncMode) -> bluer::Result<()> {
        let mut buf = EAR2_ANC;
        buf[9] = mode.into();

        if let Some(stream) = &mut self.stream {
            stream.write_all(&buf).await?;
            Ok(())
        } else {
            Err(Error {
                kind: ErrorKind::ConnectionAttemptFailed,
                message: "set_acn failed because of no connection".to_string(),
            })
        }
    }

    pub async fn set_low_latency(&mut self, mode: bool) -> bluer::Result<()> {
        if let Some(stream) = &mut self.stream {
            if mode {
                stream.write_all(&EAR2_LOW_LAG_ON).await?;
            } else {
                stream.write_all(&EAR2_LOW_LAG_OFF).await?;
            }
            Ok(())
        } else {
            Err(Error {
                kind: ErrorKind::ConnectionAttemptFailed,
                message: "set_low_latency failed because of no connection".to_string(),
            })
        }
    }

    pub async fn set_in_ear_detection(&mut self, mode: bool) -> bluer::Result<()> {
        if let Some(stream) = &mut self.stream {
            if mode {
                stream.write_all(&EAR2_IN_EAR_DETECTION_ON).await?;
            } else {
                stream.write_all(&EAR2_IN_EAR_DETECTION_OFF).await?;
            }
            Ok(())
        } else {
            Err(Error {
                kind: ErrorKind::ConnectionAttemptFailed,
                message: "set_in_ear_detection failed because of no connection".to_string(),
            })
        }
    }
}
