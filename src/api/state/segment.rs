use serde::Serialize;

pub type Segments = Vec<Segment>;

pub struct Segment {
    /// Zero-indexed ID of the segment. If omitted, the ID will be inferred from the position in the `seg` array
    id: Option<u8>,

    /// The LED that the segment starts at.
    start: u8,

    /// The LED that the segment ends at.
    stop: u8,

    /// Start row from top-left corner of the matrix.
    startY: u8,

    /// Stop row from the top-left corner of the matrix.
    stopY: u8,

    /// The total length of the segment. Omitted if `stop` is being returned.
    len: u32,

    fx: Fx,
    // TODO: A whole lot more properties
}

// ? Going a little overboard with u8s here. Should review at some point.

enum Fx {
    ID(u8),
    Increment,
    Decrement,
    Random,
}

impl Serialize for Fx {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Fx::ID(id) => serializer.serialize_u8(*id),
            Fx::Increment => serializer.serialize_str("~"),
            Fx::Decrement => serializer.serialize_str("~-"),
            Fx::Random => serializer.serialize_str("r"),
        }
    }
}
