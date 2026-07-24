use super::{collect, dedup, report, t::Config};

pub fn run() -> bool {
    let config = Config::new();
    let candidates = collect::all_fragments(config.min_stmts);
    let scanned = candidates.len();
    report::print(scanned, dedup::groups(candidates, &config))
}
