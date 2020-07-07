mod objects;
mod utils;

use objects::graph::Graph;
use utils::{Error, Result};

#[no_mangle]
pub extern "C" fn get_graph_from_url(url: *const u8, len: usize) -> Result<Graph, Error> {
    internal_get(url, len)
}

fn internal_get<T>(url: *const u8, len: usize) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    let url_slice = unsafe { std::slice::from_raw_parts(url, len) };
    let utf8_url = match std::str::from_utf8(url_slice) {
        Ok(str) => str,
        Err(_) => return Result::Err(Error::InvalidUrl),
    };

    let result = match reqwest::blocking::get(utf8_url) {
        Ok(result) => result,
        Err(_) => return Result::Err(Error::NetworkError),
    };

    let result: T = match result.json() {
        Ok(result) => result,
        Err(_) => return Result::Err(Error::InvalidResponse),
    };

    Result::Ok(result)
}
