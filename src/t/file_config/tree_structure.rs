use serde::Deserialize;

use super::Imports;

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct TreeStructure {
    #[serde(default)]
    pub imports: Imports,
}
