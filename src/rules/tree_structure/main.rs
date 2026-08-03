use {
    super::{common_items, common_nesting, imports, t_common},
    crate::rules::c::parts,
};

pub fn run() -> bool {
    let reports = vec![
        imports::part(),
        common_items::part(),
        common_nesting::part(),
        t_common::part(),
    ];
    parts::print("tree-structure", reports)
}
