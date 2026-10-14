use {
    super::{collect, dedup, report, t::Config},
    crate::rules::c::scan,
};

pub fn run() -> bool {
    let config = Config::new();
    let (scanned, groups) = scan::per_src_dir(
        |dir_name| collect::fragments(dir_name, config.min_stmts),
        |candidates| dedup::groups(candidates, &config),
    );
    report::print(scanned, groups)
}
