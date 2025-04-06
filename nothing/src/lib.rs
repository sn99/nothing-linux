use crate::anc::AncMode;

pub mod anc;
pub mod connect;
pub mod nothing_ear_2;

pub trait Nothing {
    fn get_address(&self) -> impl std::future::Future<Output = Option<String>> + Send;
    fn get_firmware_version(&self) -> impl std::future::Future<Output = Option<String>> + Send;
    fn get_serial_number(&self) -> impl std::future::Future<Output = Option<String>> + Send;

    fn set_anc_mode(
        &mut self,
        mode: AncMode,
    ) -> impl std::future::Future<Output = Result<(), bluer::Error>> + Send;

    fn set_low_latency_mode(
        &mut self,
        mode: bool,
    ) -> impl std::future::Future<Output = Result<(), bluer::Error>> + Send;

    fn set_in_ear_detection_mode(
        &mut self,
        mode: bool,
    ) -> impl std::future::Future<Output = Result<(), bluer::Error>> + Send;

    fn try_connect(&mut self)
        -> impl std::future::Future<Output = Result<(), bluer::Error>> + Send;
}
