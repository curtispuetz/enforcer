use std::path::PathBuf;

pub struct SurfaceItem {
    pub kind: &'static str,
    pub name: String,
    pub file: PathBuf,
}
