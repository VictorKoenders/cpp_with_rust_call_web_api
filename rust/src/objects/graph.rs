use crate::utils::CompatString;

#[repr(C)]
#[derive(serde::Deserialize)]
pub struct Graph {
    pub id: i32,
    pub name: CompatString,
    pub x_label: CompatString,
    pub y_label: CompatString,
}
