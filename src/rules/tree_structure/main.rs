use {
    super::{common_items, imports, t_common},
    crate::rules::c::parts,
};

pub fn run() -> bool {
    let reports = vec![
        imports::part(),
        common_items::part(),
        t_common::part(),
    ];
    parts::print("tree-structure", reports)
}
