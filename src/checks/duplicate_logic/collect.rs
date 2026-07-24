use syn::{
    Block,
    spanned::Spanned,
    visit::{self, Visit},
};

use crate::{
    checks::c::{alpha, files, path},
    s::EXISTING_SRC_DIRS,
};

use super::t::{Candidate, Occurrence};

pub fn all_fragments(min_stmts: usize) -> (Vec<Candidate>, usize) {
    let mut walk = Walk {
        path: String::new(),
        min_stmts,
        candidates: Vec::new(),
        fns: 0,
    };
    for dir_name in EXISTING_SRC_DIRS.iter() {
        for file_path in files::rs(dir_name) {
            let file = files::ast_parse(&file_path);
            walk.path = path::rel(&file_path);
            walk.visit_file(&file);
        }
    }
    (walk.candidates, walk.fns)
}

struct Walk {
    path: String,
    min_stmts: usize,
    candidates: Vec<Candidate>,
    fns: usize,
}

impl Walk {
    fn _enumerate(&mut self, block: &Block) {
        let stmts = &block.stmts;
        let n = stmts.len();
        for start in 0..n {
            for end in (start + self.min_stmts)..=n {
                let slice = &stmts[start..end];
                self.candidates.push(Candidate {
                    canonical: alpha::canonical_block(slice),
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

    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        self.fns += 1;
        visit::visit_item_fn(self, item);
    }

    fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
        self.fns += 1;
        visit::visit_impl_item_fn(self, item);
    }
}
