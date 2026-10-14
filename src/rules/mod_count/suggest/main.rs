use {
    super::{group, label, merge, nest},
    crate::rules::{
        c::siblings,
        mod_count::t::{Config, Fix},
        t::FileImports,
    },
    std::{cmp::Reverse, collections::BTreeSet, path::Path},
};

pub fn fixes(mod_rs: &Path, config: &Config, index: &[FileImports]) -> Vec<Fix> {
    let links = siblings::of(mod_rs, index);
    let mut fixes = merge::all(mod_rs, &links, config.merge_max_lines);
    fixes.extend(nest::all(mod_rs, &links, config.max));
    fixes.extend(group::all(mod_rs, &links));
    fixes.sort_by_cached_key(_rank);
    let mut seen = BTreeSet::new();
    fixes.retain(|fix| seen.insert(_footprint(fix)));
    fixes.sort_by_key(_display_order);
    label::all(mod_rs, fixes)
}

fn _display_order(fix: &Fix) -> u8 {
    match fix {
        Fix::Nest { .. } => 0,
        Fix::Group { .. } => 1,
        Fix::Merge { .. } => 2,
    }
}

fn _footprint(fix: &Fix) -> BTreeSet<String> {
    match fix {
        Fix::Merge { from, into, .. } => BTreeSet::from([from.clone(), into.clone()]),
        Fix::Nest { owner, owned, .. } => {
            owned.iter().chain([owner]).cloned().collect()
        }
        Fix::Group { members, .. } => members.iter().cloned().collect(),
    }
}

fn _rank(fix: &Fix) -> (u8, usize, Reverse<usize>, usize, String) {
    let frees = Reverse(fix.frees());
    match fix {
        Fix::Merge { from, lines, .. } => {
            (0, 0, frees, lines.0 + lines.1, from.clone())
        }
        Fix::Nest { owner, .. } => (1, 0, frees, 0, owner.clone()),
        Fix::Group { members, repoint } => {
            (2, repoint.len(), frees, 0, members.join(","))
        }
    }
}
