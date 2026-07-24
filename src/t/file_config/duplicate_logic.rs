use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DuplicateLogic {
    #[serde(default = "_default_min_stmts")]
    pub min_stmts: usize,
    #[serde(default)]
    pub ignore: Vec<String>,
}

impl Default for DuplicateLogic {
    fn default() -> Self {
        DuplicateLogic {
            min_stmts: _default_min_stmts(),
            ignore: Vec::new(),
        }
    }
}

fn _default_min_stmts() -> usize {
    2
}
