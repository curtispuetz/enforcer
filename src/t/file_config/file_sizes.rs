use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileSizes {
    #[serde(default = "_default_num")]
    pub max: usize,
    #[serde(default)]
    pub ignore: Vec<String>,
}

impl Default for FileSizes {
    fn default() -> Self {
        FileSizes {
            max: _default_num(),
            ignore: Vec::new(),
        }
    }
}

fn _default_num() -> usize {
    99
}
