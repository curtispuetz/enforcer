use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ModCount {
    pub max: usize,
    pub ignore: Vec<String>,
}

impl Default for ModCount {
    fn default() -> Self {
        ModCount {
            max: 10,
            ignore: Vec::new(),
        }
    }
}
