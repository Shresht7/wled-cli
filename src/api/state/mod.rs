mod power;
pub use power::*;

mod segment;
pub use segment::*;

pub struct State {
    on: PowerState,
    bri: u8,
    seg: Segments,
}
