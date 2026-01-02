use serde::{Deserialize, Serialize};

pub type Segments = Vec<Segment>;

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    /// Zero-indexed ID of the segment. If omitted, the ID will be inferred from the position in the `seg` array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u8>,

    /// The LED that the segment starts at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u16>,

    /// The LED that the segment ends at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<u16>,

    /// Start row from top-left corner of the matrix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_y: Option<u16>,

    /// Stop row from the top-left corner of the matrix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_y: Option<u16>,

    /// The total length of the segment. Omitted if `stop` is being returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fx: Option<Fx>,
    // TODO: A whole lot more properties
}

// ? Going a little overboard with u8s here. Should review at some point.

#[derive(Debug)]
pub enum Fx {
    ID(u8),
    Increment,
    Decrement,
    Random,
}

impl std::fmt::Display for Fx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fx::ID(id) => write!(f, "{id}"),
            Fx::Increment => write!(f, "~"),
            Fx::Decrement => write!(f, "~-"),
            Fx::Random => write!(f, "r"),
        }
    }
}

impl std::str::FromStr for Fx {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "~" => Ok(Fx::Increment),
            "~-" => Ok(Fx::Decrement),
            "r" => Ok(Fx::Random),
            _ => Ok(Fx::ID(s.parse()?)),
        }
    }
}

impl Serialize for Fx {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Fx::ID(id) => serializer.serialize_u8(*id),
            // ? Can try &self.to_string() here instead of duplicating ~, ~- and r.
            Fx::Increment => serializer.serialize_str("~"),
            Fx::Decrement => serializer.serialize_str("~-"),
            Fx::Random => serializer.serialize_str("r"),
        }
    }
}

impl<'de> Deserialize<'de> for Fx {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FxVisitor;

        impl<'de> serde::de::Visitor<'de> for FxVisitor {
            type Value = Fx;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an integer ID or a string (~, ~-, r)")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Fx::ID(value as u8))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Fx::ID(value as u8))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "~" => Ok(Fx::Increment),
                    "~-" => Ok(Fx::Decrement),
                    "r" => Ok(Fx::Random),
                    _ => Err(E::custom(format!("Unknown string value for Fx: {}", value))),
                }
            }
        }

        deserializer.deserialize_any(FxVisitor)
    }
}
