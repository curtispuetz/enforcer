use super::{
    c_fns_used, common_items, common_reexport, import_rules, report, t_common,
};

pub fn run() -> bool {
    let parts = vec![
        import_rules::part(),
        common_items::part(),
        common_reexport::part(),
        c_fns_used::part(),
        t_common::part(),
    ];
    report::print(parts)
}
