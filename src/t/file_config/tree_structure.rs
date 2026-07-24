use serde::Deserialize;

use super::ImportRules;

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct TreeStructure {
    #[serde(default)]
    pub import_rules: ImportRules,
}
