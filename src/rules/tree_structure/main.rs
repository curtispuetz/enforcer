use super::{
    c_fns_used, common_items, common_reexport, imports, report, t_common,
};

pub fn run() -> bool {
    let parts = vec![
        imports::part(),
        common_items::part(),
        common_reexport::part(),
        c_fns_used::part(),
        t_common::part(),
    ];
    report::print(parts)
}
