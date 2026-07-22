use super::Function;

pub struct Violation {
    pub path: String,
    pub functions: Vec<Function>,
}
