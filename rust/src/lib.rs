mod compat;
mod objects;

use compat::Error;

fn internal_get<T>(url: *const u8, len: usize) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    let url_slice = unsafe { std::slice::from_raw_parts(url, len) };
    let utf8_url = std::str::from_utf8(url_slice).map_err(|_| Error::InvalidUrl)?;

    let result = reqwest::blocking::get(utf8_url).map_err(|_| Error::NetworkError)?;

    let result: T = result.json().map_err(|_| Error::InvalidResponse)?;

    Result::Ok(result)
}
