pub mod nothing_ear_2;

pub trait Nothing {
    fn get_address(&self) -> impl std::future::Future<Output = String> + Send;
    fn get_firmware_version(&self) -> impl std::future::Future<Output = String> + Send;
    fn get_serial_number(&self) -> impl std::future::Future<Output = String> + Send;
}
