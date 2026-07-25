use syn::{Block, visit::Visit};

use super::t::Scorer;

pub fn of(block: &Block) -> usize {
    let mut scorer = Scorer {
        score: 0,
        nesting: 0,
        parent_logical: None,
    };
    scorer.visit_block(block);
    scorer.score
}
