use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CognitiveComplexity {
    #[serde(default = "_default_max_complexity")]
    pub max: usize,
    #[serde(default)]
    pub ignore: Vec<String>,
}

impl Default for CognitiveComplexity {
    fn default() -> Self {
        CognitiveComplexity {
            max: _default_max_complexity(),
            ignore: Vec::new(),
        }
    }
}

fn _default_max_complexity() -> usize {
    15
}
