use {
    super::t::Scorer,
    syn::{Block, visit::Visit},
};

pub fn of(block: &Block) -> usize {
    let mut scorer = Scorer {
        score: 0,
        nesting: 0,
        parent_logical: None,
    };
    scorer.visit_block(block);
    scorer.score
}
