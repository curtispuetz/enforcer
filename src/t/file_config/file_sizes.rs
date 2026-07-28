use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FileSizes {
    pub max: usize,
    pub ignore: Vec<String>,
}

impl Default for FileSizes {
    fn default() -> Self {
        FileSizes {
            max: 99,
            ignore: Vec::new(),
        }
    }
}
