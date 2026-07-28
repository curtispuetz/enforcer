use std::collections::HashMap;

use syn::Block;

use super::{
    id,
    t::{Candidate, Config, Group, Hole, Leaf},
};

pub fn merge(candidates: Vec<Candidate>, config: &Config) -> Vec<Group> {
    let mut buckets: HashMap<Block, Vec<Candidate>> = HashMap::new();
    for candidate in candidates {
        buckets
            .entry(candidate.shape.block.clone())
            .or_default()
            .push(candidate);
    }
    let mut groups = Vec::new();
    for bucket in buckets.into_values() {
        groups.extend(_bucket_groups(bucket, config));
    }
    groups
}

// not-obvious: a skeleton bucket holds every fragment of that shape, related or
// not, so it is first split into exactly-equal clusters and those are only
// merged back together while the merged form stays within the hole budget
fn _bucket_groups(bucket: Vec<Candidate>, config: &Config) -> Vec<Group> {
    let mut exact: HashMap<Block, Vec<Candidate>> = HashMap::new();
    for candidate in bucket {
        exact
            .entry(candidate.canonical.clone())
            .or_default()
            .push(candidate);
    }
    let mut clusters: Vec<Vec<Candidate>> = exact.into_values().collect();
    clusters.sort_by_key(|cluster| _id(cluster));
    _merged(clusters, config)
        .into_iter()
        .filter_map(|cluster| _group(cluster, config))
        .collect()
}

fn _merged(clusters: Vec<Vec<Candidate>>, config: &Config) -> Vec<Vec<Candidate>> {
    let mut out: Vec<Vec<Candidate>> = Vec::new();
    for cluster in clusters {
        match out.iter_mut().find(|acc| _fits(acc, &cluster, config)) {
            Some(acc) => acc.extend(cluster),
            None => out.push(cluster),
        }
    }
    out
}

fn _fits(acc: &[Candidate], cluster: &[Candidate], config: &Config) -> bool {
    let joined: Vec<&Candidate> = acc.iter().chain(cluster.iter()).collect();
    _holes(&joined).len() <= config.max_holes
}

fn _group(cluster: Vec<Candidate>, config: &Config) -> Option<Group> {
    if cluster.len() < 2 {
        return None;
    }
    let holes = _holes(&cluster.iter().collect::<Vec<_>>());
    if holes.len() * config.min_nodes_per_hole > cluster[0].shape.nodes {
        return None;
    }
    let id = _id(&cluster);
    let occurrences = cluster.into_iter().map(|c| c.occurrence).collect();
    Some(Group {
        id,
        holes,
        occurrences,
    })
}

fn _holes(cluster: &[&Candidate]) -> Vec<Hole> {
    let Some(first) = cluster.first() else {
        return Vec::new();
    };
    let mut holes = Vec::new();
    for (index, leaf) in first.shape.leaves.iter().enumerate() {
        let values = _values_at(cluster, index);
        if values.len() > 1 {
            holes.push(Hole {
                kind: leaf.kind.clone(),
                values,
            });
        }
    }
    holes
}

fn _values_at(cluster: &[&Candidate], index: usize) -> Vec<String> {
    let mut values: Vec<String> = cluster
        .iter()
        .map(|c| _text_at(&c.shape.leaves, index))
        .collect();
    values.sort();
    values.dedup();
    values
}

fn _text_at(leaves: &[Leaf], index: usize) -> String {
    leaves.get(index).map(|l| l.text.clone()).unwrap_or_default()
}

// not-obvious: the smallest member digest keeps the id stable no matter what
// order the files were walked in, and keeps hole-free groups on the id they had
// before holes existed
fn _id(cluster: &[Candidate]) -> String {
    cluster
        .iter()
        .map(|c| id::digest(&c.canonical))
        .min()
        .unwrap_or_default()
}
