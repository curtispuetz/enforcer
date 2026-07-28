use std::collections::BTreeSet;

use super::t::{Candidate, Hole, Leaf};

pub fn of(cluster: &[&Candidate]) -> Vec<Hole> {
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
    cluster
        .iter()
        .map(|c| _text_at(&c.shape.leaves, index))
        .collect::<BTreeSet<String>>()
        .into_iter()
        .collect()
}

fn _text_at(leaves: &[Leaf], index: usize) -> String {
    leaves.get(index).map(|l| l.text.clone()).unwrap_or_default()
}
