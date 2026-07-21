pub struct Comment {
    pub line: usize,
    pub col: usize,
    pub full: String,
    pub is_block: bool,
    pub trailing: bool,
}
