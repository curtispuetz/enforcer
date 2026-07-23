use std::collections::HashMap;

use syn::ItemFn;

use super::{
    collect, normalize, report,
    t::{Duplicate, Group},
};

pub fn run() -> bool {
    let free_fns = collect::all();
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

    let groups = _groups(buckets);
    let duplicated: usize = groups.iter().map(|group| group.members.len()).sum();
    report::print(total - duplicated, groups)
}

// Keep only buckets with more than one member (the actual duplicates), sorting
// members and groups so the report is deterministic regardless of hash order.
fn _groups(buckets: HashMap<ItemFn, Vec<Duplicate>>) -> Vec<Group> {
    let mut groups: Vec<Group> = buckets
        .into_values()
        .filter(|members| members.len() > 1)
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
