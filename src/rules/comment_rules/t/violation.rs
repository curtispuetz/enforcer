pub struct Violation {
    pub path: String,
    pub comments: Vec<BadComment>,
}

pub struct BadComment {
    pub line: usize,
    pub text: String,
    pub reason: String,
}
