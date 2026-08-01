use super::{common_items, common_nesting, imports, report, t_common};

pub fn run() -> bool {
    let parts = vec![
        imports::part(),
        common_items::part(),
        common_nesting::part(),
        t_common::part(),
    ];
    report::print(parts)
}
