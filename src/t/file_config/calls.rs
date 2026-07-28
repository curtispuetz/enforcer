use serde::Deserialize;

#[derive(Deserialize, Default)]
#[serde(default, deny_unknown_fields)]
pub struct Calls {
    pub ignore: Vec<String>,
}
