use serde::Deserialize;

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ImportRules {
    #[serde(default)]
    pub ignore_export_macros: bool,
}
