use serde::Deserialize;

#[derive(Deserialize, Default)]
#[serde(default, deny_unknown_fields)]
pub struct DuplicateFns {
    pub ignore: Vec<String>,
}
