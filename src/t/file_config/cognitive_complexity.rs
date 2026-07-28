use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CognitiveComplexity {
    pub max: usize,
    pub ignore: Vec<String>,
}

impl Default for CognitiveComplexity {
    fn default() -> Self {
        CognitiveComplexity {
            max: 8,
            ignore: Vec::new(),
        }
    }
}
