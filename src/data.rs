use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct AirDust {
    pub timestamp: DateTime<Local>,
    pub vouth: u8,
    pub voutl: u8,
    pub vrefh: u8,
    pub vrefl: u8,
    pub vout: f32,
    pub dust_density: f32,
}

impl fmt::Display for AirDust {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "\"timestamp\": {:?}, \"vouth\": {:.0}, \"voutl\": {:.0}, \"vrefh\": {:.0}, \"vrefl\": {:.0}, \"vout\": {:.3}, \"dust_density\": {:.3}",
            self.timestamp.format("%Y/%m/%d %H:%M:%S").to_string(),
            self.vouth,
            self.voutl,
            self.vrefh,
            self.vrefl,
            self.vout,
            self.dust_density
        )
    }
}

