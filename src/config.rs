use std::collections::HashSet;

#[derive(Default)]
pub struct Config {
    pub ignore_exported_macros: bool,
    pub exported_macros: HashSet<String>,
}
