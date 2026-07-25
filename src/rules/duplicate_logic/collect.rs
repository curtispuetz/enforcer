use syn::{
    Block, Stmt,
    spanned::Spanned,
    visit::{self, Visit},
    visit_mut::VisitMut,
};

use crate::{
    rules::c::{alpha, files, path},
    s::EXISTING_SRC_DIRS,
};

use super::t::{Candidate, Occurrence};

pub fn all_fragments(min_stmts: usize) -> Vec<Candidate> {
    let mut walk = Walk {
        path: String::new(),
        min_stmts,
        candidates: Vec::new(),
    };
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for file_path in files::rs(dir_name) {
            let file = files::ast_parse(&file_path);
            walk.path = path::rel(&file_path);
            walk.visit_file(&file);
        }
    }
    walk.candidates
}

struct Walk {
    path: String,
    min_stmts: usize,
    candidates: Vec<Candidate>,
}

impl Walk {
    fn _enumerate(&mut self, block: &Block) {
        let stmts = &block.stmts;
        let n = stmts.len();
        for start in 0..n {
            for end in (start + self.min_stmts)..=n {
                let slice = &stmts[start..end];
                self.candidates.push(Candidate {
                    canonical: _canonicalize_block(slice),
                    occurrence: Occurrence {
                        path: self.path.clone(),
                        start: slice[0].span().start().line,
                        end: slice[slice.len() - 1].span().end().line,
                    },
                });
            }
        }
    }
}

impl<'ast> Visit<'ast> for Walk {
    fn visit_block(&mut self, block: &'ast Block) {
        self._enumerate(block);
        visit::visit_block(self, block);
    }
}

fn _canonicalize_block(stmts: &[Stmt]) -> Block {
    let mut out = Block {
        brace_token: Default::default(),
        stmts: stmts.to_vec(),
    };
    alpha::canonicalize(
        &mut out,
        |c, node| c.visit_block(node),
        |r, node| r.visit_block_mut(node),
    );
    out
}
