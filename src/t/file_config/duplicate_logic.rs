use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DuplicateLogic {
    pub min_stmts: usize,
    pub max_holes: usize,
    pub min_nodes_per_hole: usize,
    pub ignore: Vec<String>,
}

impl Default for DuplicateLogic {
    fn default() -> Self {
        DuplicateLogic {
            min_stmts: 2,
            max_holes: 4,
            min_nodes_per_hole: 16,
            ignore: Vec::new(),
        }
    }
}
