use crate::anc::AncMode;

pub mod anc;
pub mod connect;
pub mod nothing_ear_2;

pub trait Nothing {
    fn get_address(&self) -> impl std::future::Future<Output = String> + Send;
    fn get_firmware_version(&self) -> impl std::future::Future<Output = String> + Send;
    fn get_serial_number(&self) -> impl std::future::Future<Output = String> + Send;

    fn set_anc_mode(
        &self,
        mode: AncMode,
    ) -> impl std::future::Future<Output = Result<(), bluer::Error>> + Send;

    fn set_low_latency_mode(
        &self,
        mode: bool,
    ) -> impl std::future::Future<Output = Result<(), bluer::Error>> + Send;

    fn set_in_ear_detection_mode(
        &self,
        mode: bool,
    ) -> impl std::future::Future<Output = Result<(), bluer::Error>> + Send;
}
