pub enum Fix {
    Merge {
        from: String,
        into: String,
        lines: (usize, usize),
    },
    Nest {
        owner: String,
        owned: Vec<String>,
        room: Option<usize>,
    },
    Group {
        members: Vec<String>,
        repoint: Vec<String>,
    },
}

impl Fix {
    pub fn frees(&self) -> usize {
        match self {
            Fix::Merge { .. } => 1,
            Fix::Nest { owned, .. } => owned.len(),
            Fix::Group { members, .. } => members.len().saturating_sub(1),
        }
    }
}
