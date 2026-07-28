use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DuplicateLogic {
    #[serde(default = "_default_min_stmts")]
    pub min_stmts: usize,
    #[serde(default = "_default_max_holes")]
    pub max_holes: usize,
    #[serde(default = "_default_min_nodes_per_hole")]
    pub min_nodes_per_hole: usize,
    #[serde(default)]
    pub ignore: Vec<String>,
}

impl Default for DuplicateLogic {
    fn default() -> Self {
        DuplicateLogic {
            min_stmts: _default_min_stmts(),
            max_holes: _default_max_holes(),
            min_nodes_per_hole: _default_min_nodes_per_hole(),
            ignore: Vec::new(),
        }
    }
}

fn _default_min_stmts() -> usize {
    2
}

fn _default_max_holes() -> usize {
    1
}

fn _default_min_nodes_per_hole() -> usize {
    20
}
