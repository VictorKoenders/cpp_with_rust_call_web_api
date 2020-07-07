use serde::de::Visitor;
use std::result::Result as StdResult;

#[repr(C)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

#[repr(C)]
pub enum Error {
    InvalidUrl,
    NetworkError,
    InvalidResponse,
}

#[repr(C)]
pub struct CompatString {
    pub ptr: *const u8,
    pub len: usize,
}

impl<'de> serde::Deserialize<'de> for CompatString {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(CompatStringVisitor)
    }
}

struct CompatStringVisitor;

impl<'de> Visitor<'de> for CompatStringVisitor {
    type Value = CompatString;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Expecting String")
    }

    fn visit_str<E>(self, v: &str) -> StdResult<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let owned = v.to_owned();
        self.visit_string(owned)
    }

    fn visit_string<E>(self, v: String) -> StdResult<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let ptr = v.as_ptr();
        let len = v.len();
        std::mem::forget(v);

        Ok(CompatString { ptr, len })
    }
}
