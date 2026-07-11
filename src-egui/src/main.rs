use eframe::egui;

use async_worker::{async_worker, DeviceInfo, EarCmd, EarResponse};
use nothing::anc::AncMode;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

mod async_worker;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([200.0, 400.0]),
        ..Default::default()
    };

    let (cmd_tx, cmd_rx) = tokio::sync::mpsc::unbounded_channel();
    let (response_tx, response_rx) = tokio::sync::mpsc::unbounded_channel();

    eframe::run_native(
        "nothing-linux",
        options,
        Box::new(|cc| {
            let ctx = cc.egui_ctx.clone();

            std::thread::spawn(|| {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();

                rt.block_on(async {
                    async_worker(cmd_rx, response_tx, ctx).await;
                });
            });

            Ok(Box::new(MyApp::new(cmd_tx, response_rx)))
        }),
    )
}

#[derive(PartialEq, Eq)]
pub enum UiAnc {
    On,
    Transparency,
    Off,
    Unknown,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UiAncMode {
    High,
    Mid,
    Low,
    Adaptive,
}

impl UiAncMode {
    fn label(&self) -> &str {
        match self {
            UiAncMode::High => "High",
            UiAncMode::Mid => "Mid",
            UiAncMode::Low => "Low",
            UiAncMode::Adaptive => "Adaptive",
        }
    }
}

impl From<&UiAncMode> for AncMode {
    fn from(value: &UiAncMode) -> Self {
        match value {
            UiAncMode::High => AncMode::High,
            UiAncMode::Mid => AncMode::Mid,
            UiAncMode::Low => AncMode::Low,
            UiAncMode::Adaptive => AncMode::Adaptive,
        }
    }
}

#[derive(PartialEq, Eq)]
enum UiLowLatency {
    On,
    Off,
    Unknown,
}

#[derive(PartialEq, Eq)]
enum UiInEarDetection {
    On,
    Off,
    Unknown,
}

struct MyApp {
    tx: UnboundedSender<EarCmd>,
    rx: UnboundedReceiver<EarResponse>,
    device_info: Option<DeviceInfo>,
    error: Option<String>,
    anc: UiAnc,
    anc_mode: UiAncMode,
    low_latency_mode: UiLowLatency,
    in_ear_detection_mode: UiInEarDetection,
}

impl MyApp {
    fn new(tx: UnboundedSender<EarCmd>, rx: UnboundedReceiver<EarResponse>) -> Self {
        Self {
            tx,
            rx,
            device_info: None,
            error: None,
            anc: UiAnc::Unknown,
            anc_mode: UiAncMode::Adaptive,
            low_latency_mode: UiLowLatency::Unknown,
            in_ear_detection_mode: UiInEarDetection::Unknown,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(response) = self.rx.try_recv() {
            match response {
                EarResponse::DeviceInfo(device_info) => {
                    self.device_info = Some(device_info);
                }
                EarResponse::Error(err) => {
                    self.error = Some(err);
                }
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Nothing Ear Manager");
            if let Some(err) = &self.error {
                ui.colored_label(ctx.style().visuals.error_fg_color, err);
            }
            ui.separator();
            if let Some(device_info) = &self.device_info {
                egui::Grid::new("DeviceInfo").num_columns(2).show(ui, |ui| {
                    ui.label("Address:");
                    ui.label(&device_info.address);
                    ui.end_row();

                    ui.label("Firmware:");
                    ui.label(&device_info.firmware_version);
                    ui.end_row();

                    ui.label("Serial:");
                    ui.label(&device_info.serial_number);
                    ui.end_row();
                });
            } else {
                ui.label("Waiting for device...");
                return;
            }
            ui.separator();
            egui::Grid::new("Anc").num_columns(2).show(ui, |ui| {
                ui.label("ANC:");
                ui.vertical(|ui| {
                    if ui.radio_value(&mut self.anc, UiAnc::Off, "Off").clicked() {
                        self.tx
                            .send(EarCmd::AncMode(AncMode::Off))
                            .expect("sending EarCmd");
                    }
                    if ui
                        .radio_value(&mut self.anc, UiAnc::Transparency, "Transparency")
                        .clicked()
                    {
                        self.tx
                            .send(EarCmd::AncMode(AncMode::Transparency))
                            .expect("sending EarCmd");
                    }
                    if ui.radio_value(&mut self.anc, UiAnc::On, "On").clicked() {
                        self.tx
                            .send(EarCmd::AncMode((&self.anc_mode).into()))
                            .expect("sending EarCmd");
                    }
                });
                ui.end_row();
            });
            ui.separator();
            ui.add_enabled_ui(self.anc == UiAnc::On, |ui| {
                egui::Grid::new("AncMode").num_columns(2).show(ui, |ui| {
                    ui.label("ANC Mode:");
                    ui.vertical(|ui| {
                        for mode in [
                            UiAncMode::High,
                            UiAncMode::Mid,
                            UiAncMode::Low,
                            UiAncMode::Adaptive,
                        ] {
                            if ui
                                .radio_value(&mut self.anc_mode, mode, mode.label())
                                .clicked()
                            {
                                self.tx
                                    .send(EarCmd::AncMode((&mode).into()))
                                    .expect("sending EarCmd");
                            }
                        }
                    });
                    ui.end_row();
                });
            });
            ui.separator();
            egui::Grid::new("LowLatency").num_columns(2).show(ui, |ui| {
                ui.label("Low Latency:");
                ui.vertical(|ui| {
                    if ui
                        .radio_value(&mut self.low_latency_mode, UiLowLatency::Off, "Off")
                        .clicked()
                    {
                        self.tx
                            .send(EarCmd::LowLatency(false))
                            .expect("sending EarCmd");
                    }
                    if ui
                        .radio_value(&mut self.low_latency_mode, UiLowLatency::On, "On")
                        .clicked()
                    {
                        self.tx
                            .send(EarCmd::LowLatency(true))
                            .expect("sending EarCmd");
                    }
                });
                ui.end_row();
            });
            ui.separator();
            egui::Grid::new("InEarDetection")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("In Ear Detection:");
                    ui.vertical(|ui| {
                        if ui
                            .radio_value(
                                &mut self.in_ear_detection_mode,
                                UiInEarDetection::Off,
                                "Off",
                            )
                            .clicked()
                        {
                            self.tx
                                .send(EarCmd::InEarDetection(false))
                                .expect("sending EarCmd");
                        }
                        if ui
                            .radio_value(
                                &mut self.in_ear_detection_mode,
                                UiInEarDetection::On,
                                "On",
                            )
                            .clicked()
                        {
                            self.tx
                                .send(EarCmd::InEarDetection(true))
                                .expect("sending EarCmd");
                        }
                    });
                    ui.end_row();
                });
        });
    }
}
