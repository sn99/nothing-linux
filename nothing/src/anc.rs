use strum::Display;

#[derive(Display, Debug, Clone, Copy, PartialEq, Eq)]
#[strum(serialize_all = "kebab-case")]
pub enum AncMode {
    High,
    Mid,
    Low,
    Adaptive,
    Transparency,
    Off,
}

impl From<AncMode> for u8 {
    fn from(val: AncMode) -> Self {
        match val {
            AncMode::Adaptive => 4,
            AncMode::High => 1,
            AncMode::Mid => 2,
            AncMode::Low => 3,
            AncMode::Transparency => 7,
            AncMode::Off => 5,
        }
    }
}
