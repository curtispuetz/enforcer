use std::collections::HashMap;

use syn::ItemFn;

use super::{
    collect, normalize, report,
    t::{Config, Duplicate, Group},
};

pub fn run() -> bool {
    let config = Config::new();
    let free_fns = collect::all_codebase_free_fns();
    let total = free_fns.len();

    let mut buckets: HashMap<ItemFn, Vec<Duplicate>> = HashMap::new();
    for function in free_fns {
        let key = normalize::canonical(&function.item);
        buckets.entry(key).or_default().push(Duplicate {
            path: function.path,
            name: function.name,
            line: function.line,
        });
    }

    let groups = _groups(buckets, &config);
    let duplicated: usize = groups.iter().map(|group| group.members.len()).sum();
    report::print(total - duplicated, groups)
}

fn _groups(buckets: HashMap<ItemFn, Vec<Duplicate>>, config: &Config) -> Vec<Group> {
    let mut groups: Vec<Group> = buckets
        .into_values()
        .filter(|members| members.len() > 1 && !_ignored(members, config))
        .map(|mut members| {
            members.sort_by(|a, b| (&a.path, a.line).cmp(&(&b.path, b.line)));
            Group { members }
        })
        .collect();
    groups.sort_by(|a, b| {
        let first = &a.members[0];
        let second = &b.members[0];
        (&first.path, first.line).cmp(&(&second.path, second.line))
    });
    groups
}

fn _ignored(members: &[Duplicate], config: &Config) -> bool {
    members.iter().any(|member| {
        config
            .ignore
            .contains(&format!("{}::{}", member.path, member.name))
    })
}
