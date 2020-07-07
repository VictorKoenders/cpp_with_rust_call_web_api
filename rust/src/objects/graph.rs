use crate::compat::{Error, Result, String};

#[repr(C)]
#[derive(serde::Deserialize)]
pub struct Graph {
    pub id: i32,
    pub name: String,
    pub x_label: String,
    pub y_label: String,
}

#[no_mangle]
pub extern "C" fn get_graph_from_url(url: *const u8, len: usize) -> Result<Graph, Error> {
    crate::internal_get(url, len).into()
}
