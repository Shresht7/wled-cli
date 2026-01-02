mod power;
pub use power::*;

mod segment;
pub use segment::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    /// Whether the device's power is on or off
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<PowerState>,

    /// The brightness level of the device. Ranges from 0 to 255.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bri: Option<u8>,

    /// An array of LED segments. Segments are groups of LEDs on the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seg: Option<Segments>,
    // TODO: The rest of the stuff
}
