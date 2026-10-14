use super::Fix;

pub struct Violation {
    pub module: String,
    pub count: usize,
    pub fixes: Vec<Fix>,
}
