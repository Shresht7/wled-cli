use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub enum PowerState {
    /// Turn the device on
    On,
    /// Turn the device off
    Off,
    /// Toggle the device on or off
    Toggle,
}

impl std::fmt::Display for PowerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PowerState::On => write!(f, "on"),
            PowerState::Off => write!(f, "off"),
            PowerState::Toggle => write!(f, "toggle"),
        }
    }
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

impl<'de> Deserialize<'de> for PowerState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PowerStateVisitor;

        impl<'de> serde::de::Visitor<'de> for PowerStateVisitor {
            type Value = PowerState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a boolean or string (t)")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if value {
                    Ok(PowerState::On)
                } else {
                    Ok(PowerState::Off)
                }
            }

            // Fallback for string inputs if ever needed
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "t" => Ok(PowerState::Toggle),
                    "on" | "On" => Ok(PowerState::On),
                    "off" | "Off" => Ok(PowerState::Off),
                    _ => Err(E::custom(format!(
                        "Unknown string value for PowerState: {}",
                        value
                    ))),
                }
            }
        }

        deserializer.deserialize_any(PowerStateVisitor)
    }
}
