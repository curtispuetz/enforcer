use super::{c_fn_location, common_items, common_nesting, imports, report, t_common};

pub fn run() -> bool {
    let parts = vec![
        imports::part(),
        common_items::part(),
        common_nesting::part(),
        c_fn_location::part(),
        t_common::part(),
    ];
    report::print(parts)
}
