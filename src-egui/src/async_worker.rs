use eframe::egui;
use nothing::{anc::AncMode, nothing_ear_2::Ear2};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub enum EarCmd {
    AncMode(AncMode),
    LowLatency(bool),
    InEarDetection(bool),
}

pub enum EarResponse {
    DeviceInfo(DeviceInfo),
    Error(String),
}

pub struct DeviceInfo {
    pub address: String,
    pub firmware_version: String,
    pub serial_number: String,
}

pub async fn async_worker(
    mut rx: UnboundedReceiver<EarCmd>,
    tx: UnboundedSender<EarResponse>,
    ctx: egui::Context,
) {
    let send_response = |response: EarResponse| async {
        tx.send(response).expect("sending EarResponse");
        ctx.request_repaint();
    };
    let ear_2 = match Ear2::new().await {
        Ok(ear_2) => {
            send_response(EarResponse::DeviceInfo(DeviceInfo {
                address: ear_2.address.clone(),
                firmware_version: ear_2.firmware_version.clone(),
                serial_number: ear_2.serial_number.clone(),
            }))
            .await;
            ear_2
        }
        Err(err) => {
            send_response(EarResponse::Error(err.to_string())).await;
            return;
        }
    };
    while let Some(cmd) = rx.recv().await {
        match cmd {
            EarCmd::AncMode(anc_mode) => {
                if let Err(err) = ear_2.set_anc(anc_mode).await {
                    send_response(EarResponse::Error(err.to_string())).await;
                }
            }
            EarCmd::LowLatency(mode) => {
                if let Err(err) = ear_2.set_low_latency(mode).await {
                    send_response(EarResponse::Error(err.to_string())).await;
                }
            }
            EarCmd::InEarDetection(mode) => {
                if let Err(err) = ear_2.set_in_ear_detection(mode).await {
                    send_response(EarResponse::Error(err.to_string())).await;
                }
            }
        }
    }
}
