use {
    super::{
        t::{FileImports, Folder, Member},
        verdict,
    },
    std::collections::{BTreeMap, HashSet},
};

pub fn group<'a>(
    folder: &'a Folder,
    flagged: &HashSet<&str>,
    index: &[FileImports],
) -> Vec<Vec<&'a Member>> {
    let members: Vec<&Member> = folder
        .members
        .iter()
        .filter(|m| flagged.contains(m.name.as_str()))
        .collect();
    let mut labels: Vec<usize> = (0..members.len()).collect();
    for (b, member) in members.iter().enumerate() {
        for file in verdict::importers(folder, member, index) {
            if let Some(a) = members.iter().position(|m| file.module.starts_with(&m.segments)) {
                _merge(&mut labels, a, b);
            }
        }
    }
    _collect(members, &labels)
}

pub fn importers<'a>(
    folder: &Folder,
    cluster: &[&Member],
    index: &'a [FileImports],
) -> Vec<&'a FileImports> {
    let mut ret: Vec<&FileImports> = cluster
        .iter()
        .flat_map(|member| verdict::importers(folder, member, index))
        .filter(|file| !cluster.iter().any(|m| file.module.starts_with(&m.segments)))
        .collect();
    ret.sort_by(|a, b| a.path.cmp(&b.path));
    ret.dedup_by(|a, b| a.path == b.path);
    ret
}

fn _merge(labels: &mut [usize], a: usize, b: usize) {
    let (from, to) = (labels[a], labels[b]);
    for label in labels.iter_mut().filter(|l| **l == from) {
        *label = to;
    }
}

fn _collect<'a>(members: Vec<&'a Member>, labels: &[usize]) -> Vec<Vec<&'a Member>> {
    let mut groups: BTreeMap<usize, Vec<&Member>> = BTreeMap::new();
    for (member, label) in members.into_iter().zip(labels) {
        groups.entry(*label).or_default().push(member);
    }
    groups.into_values().collect()
}
