use super::{collect, dedup, report, t::Config};

pub fn run() -> bool {
    let config = Config::new();
    let (candidates, fns) = collect::all_fragments(config.min_stmts);
    let groups = dedup::groups(candidates, &config);
    report::print(fns, groups)
}
