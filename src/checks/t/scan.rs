pub struct Results<T> {
    pub passed: usize,
    pub violations: Vec<T>,
}

impl<T> Results<T> {
    pub fn new() -> Self {
        Self {
            passed: 0,
            violations: Vec::new(),
        }
    }
}
