use std::collections::HashMap;

use syn::Block;

use super::{
    id,
    t::{Candidate, Config, Group, Occurrence},
};

pub fn groups(candidates: Vec<Candidate>, config: &Config) -> Vec<Group> {
    let mut buckets: HashMap<Block, Vec<Occurrence>> = HashMap::new();
    for candidate in candidates {
        buckets
            .entry(candidate.canonical)
            .or_default()
            .push(candidate.occurrence);
    }

    let raw: Vec<(String, Vec<Occurrence>)> = buckets
        .into_iter()
        .filter(|(_, occs)| occs.len() > 1)
        .map(|(block, occs)| (id::digest(&block), occs))
        .collect();

    let spans = _all_spans(&raw);
    let mut kept = _survivors(raw, &spans, config);
    kept.sort_by(|a, b| {
        let x = &a.occurrences[0];
        let y = &b.occurrences[0];
        (&x.path, x.start).cmp(&(&y.path, y.start))
    });
    kept
}

fn _all_spans(raw: &[(String, Vec<Occurrence>)]) -> Vec<(usize, Occurrence)> {
    let mut spans = Vec::new();
    for (_, occs) in raw {
        for occ in occs {
            spans.push((occs.len(), occ.clone()));
        }
    }
    spans
}

// not-obvious: a sub-fragment that only ever appears inside a larger, equally-or-
// more-duplicated fragment is redundant noise, so we drop the dominated spans and
// keep only maximal clones
fn _survivors(
    raw: Vec<(String, Vec<Occurrence>)>,
    spans: &[(usize, Occurrence)],
    config: &Config,
) -> Vec<Group> {
    let mut kept = Vec::new();
    for (id, occs) in raw {
        if config.ignore.contains(&id) {
            continue;
        }
        let support = occs.len();
        let occurrences: Vec<Occurrence> = occs
            .into_iter()
            .filter(|occ| !_dominated(occ, support, spans))
            .collect();
        if occurrences.len() > 1 {
            kept.push(Group { id, occurrences });
        }
    }
    kept
}

fn _dominated(occ: &Occurrence, support: usize, spans: &[(usize, Occurrence)]) -> bool {
    spans
        .iter()
        .any(|(other_support, other)| *other_support >= support && _contains(other, occ))
}

fn _contains(outer: &Occurrence, inner: &Occurrence) -> bool {
    outer.path == inner.path
        && outer.start <= inner.start
        && outer.end >= inner.end
        && (outer.start < inner.start || outer.end > inner.end)
}
