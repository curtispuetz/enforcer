use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModCount {
    #[serde(default = "_default_max_mods")]
    pub max: usize,
    #[serde(default)]
    pub ignore: Vec<String>,
}

impl Default for ModCount {
    fn default() -> Self {
        ModCount {
            max: _default_max_mods(),
            ignore: Vec::new(),
        }
    }
}

fn _default_max_mods() -> usize {
    10
}
