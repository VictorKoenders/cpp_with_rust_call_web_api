use serde::de::Visitor;
use std::result::Result as StdResult;
use std::string::String as StdString;

#[repr(C)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> From<StdResult<T, E>> for Result<T, E> {
    fn from(r: StdResult<T, E>) -> Result<T, E> {
        match r {
            Ok(t) => Result::Ok(t),
            Err(e) => Result::Err(e),
        }
    }
}

#[repr(C)]
pub enum Error {
    InvalidUrl,
    NetworkError,
    InvalidResponse,
}

#[repr(C)]
pub struct String {
    pub ptr: *const u8,
    pub len: usize,
}

impl From<StdString> for String {
    fn from(s: StdString) -> Self {
        let ptr = s.as_ptr();
        let len = s.len();
        std::mem::forget(s);

        Self { ptr, len }
    }
}

impl<'de> serde::Deserialize<'de> for String {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(StringVisitor)
    }
}

struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = String;
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

    fn visit_string<E>(self, v: StdString) -> StdResult<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.into())
    }
}
