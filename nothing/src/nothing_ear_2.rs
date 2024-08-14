use crate::Nothing;

use std::process;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct Ear2 {
    pub address: String,
    pub firmware_version: String,
    pub serial_number: String,
    // stream: bluer::rfcomm::Stream,
}

impl Nothing for Ear2 {
    async fn get_address(&self) -> String {
        self.address.clone()
    }
    async fn get_firmware_version(&self) -> String {
        self.firmware_version.clone()
    }
    async fn get_serial_number(&self) -> String {
        self.serial_number.clone()
    }
}

impl Ear2 {
    pub async fn new() -> bluer::Result<Self> {
        let Ok(session) = bluer::Session::new().await else {
            eprintln!("Could not connect to Bluetooth daemon over DBus.");
            process::exit(1);
        };
        let Ok(adapter) = session.default_adapter().await else {
            eprintln!("No Bluetooth adapter found.");
            process::exit(1);
        };
        if adapter.set_powered(true).await.is_err() {
            eprintln!("Bluetooth seems to be disabled. Please enable Bluetooth.");
            process::exit(1);
        }

        let device_addresses = adapter.device_addresses().await?;
        let Some(&ear2_address) = device_addresses
            .iter()
            .find(|&addr| matches!(addr, bluer::Address([0x2C, 0xBE, 0xEB, _, _, _])))
        else {
            eprintln!("Couldn't find any Ear (2) devices connected. Make sure you're paired with your Ear (2).");
            process::exit(1);
        };

        let Ok(mut stream) = bluer::rfcomm::Stream::connect(bluer::rfcomm::SocketAddr {
            addr: ear2_address,
            channel: 15,
        })
        .await
        else {
            eprintln!("Couldn't connect to Ear (2). Make sure you're paired.");
            process::exit(1);
        };

        // get firmware version
        stream
            .write_all(&[0x55, 0x60, 0x01, 0x42, 0xc0, 0x00, 0x00, 0x03, 0xe0, 0xd1])
            .await?;
        let mut buf = [0_u8; 8];
        stream.read_exact(&mut buf).await?;
        let version_str_len: usize = buf[5].try_into().unwrap();
        let mut buf = vec![0_u8; version_str_len + 2];
        stream.read_exact(&mut buf).await?;
        let version = String::from_utf8_lossy(&buf[..version_str_len]);

        // get serial number
        stream
            .write_all(&[0x55, 0x60, 0x01, 0x06, 0xc0, 0x00, 0x00, 0x05, 0x90, 0xdc])
            .await?;
        let mut buf = [0_u8; 64 + 64 + 18];
        stream.read_exact(&mut buf).await?;
        let serial = String::from_utf8_lossy(&buf[37..53]);

        Ok(Self {
            address: ear2_address.to_string(),
            firmware_version: version.to_string(),
            serial_number: serial.to_string(),
            // stream,
        })
    }
}
