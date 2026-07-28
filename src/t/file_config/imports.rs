use serde::Deserialize;

#[derive(Deserialize, Default)]
#[serde(default, deny_unknown_fields)]
pub struct Imports {
    pub ignore_export_macros: bool,
}
