use clap::Subcommand;
use serde::Serialize;

#[derive(Debug, Subcommand)]
pub enum PowerState {
    /// Turn the device on
    On,
    /// Turn the device off
    Off,
    /// Toggle the device on or off
    Toggle,
}

impl Serialize for PowerState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            PowerState::On => serializer.serialize_bool(true),
            PowerState::Off => serializer.serialize_bool(false),
            PowerState::Toggle => serializer.serialize_str("t"),
        }
    }
}
