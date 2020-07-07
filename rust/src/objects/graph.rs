use crate::utils::{CompatString, Error, Result};

#[repr(C)]
#[derive(serde::Deserialize)]
pub struct Graph {
    pub id: i32,
    pub name: CompatString,
    pub x_label: CompatString,
    pub y_label: CompatString,
}

#[no_mangle]
pub extern "C" fn get_graph_from_url(url: *const u8, len: usize) -> Result<Graph, Error> {
    crate::internal_get(url, len)
}
