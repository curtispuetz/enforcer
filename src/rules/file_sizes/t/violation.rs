pub struct Violation {
    pub path: String,
    pub lines: usize,
    pub fixes: Vec<String>,
}
