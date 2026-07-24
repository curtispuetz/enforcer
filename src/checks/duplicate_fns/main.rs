use crate::checks::c::alpha;
use std::collections::HashMap;
use syn::{visit::Visit, visit_mut::VisitMut};

use proc_macro2::Span;
use syn::{Ident, ItemFn, Visibility};

use super::{
    collect, report,
    t::{Config, Duplicate, Group},
};

pub fn run() -> bool {
    let config = Config::new();
    let fns = collect::all_codebase_fns();
    let total = fns.len();

    let mut buckets: HashMap<ItemFn, Vec<Duplicate>> = HashMap::new();
    for function in fns {
        let key = _canonicalize_item_fn(&function.item);
        buckets.entry(key).or_default().push(Duplicate {
            path: function.path,
            name: function.name,
            line: function.line,
        });
    }

    report::print(total, _groups(buckets, &config))
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

fn _canonicalize_item_fn(item: &ItemFn) -> ItemFn {
    let mut out = item.clone();
    alpha::canonicalize(
        &mut out,
        |c, node| c.visit_item_fn(node),
        |r, node| r.visit_item_fn_mut(node),
    );
    out.attrs.clear();
    out.vis = Visibility::Inherited;
    out.sig.ident = Ident::new("__fn", Span::call_site());
    out
}
