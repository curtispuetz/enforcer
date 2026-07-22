#[derive(Clone, Copy, PartialEq)]
pub enum Logical {
    And,
    Or,
}

pub struct Scorer {
    pub score: usize,
    pub nesting: usize,
    pub parent_logical: Option<Logical>,
}
