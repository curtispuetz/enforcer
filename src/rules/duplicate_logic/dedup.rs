use super::{
    cluster,
    t::{Candidate, Config, Group, Occurrence},
};

pub fn groups(candidates: Vec<Candidate>, config: &Config) -> Vec<Group> {
    let raw = cluster::merge(candidates, config);
    let spans = _all_spans(&raw);
    let mut kept = _survivors(raw, &spans, config);
    kept.sort_by(|a, b| {
        let x = &a.occurrences[0];
        let y = &b.occurrences[0];
        (&x.path, x.start).cmp(&(&y.path, y.start))
    });
    kept
}

fn _all_spans(raw: &[Group]) -> Vec<(usize, Occurrence)> {
    let mut spans = Vec::new();
    for group in raw {
        for occ in &group.occurrences {
            spans.push((group.occurrences.len(), occ.clone()));
        }
    }
    spans
}

// not-obvious: a sub-fragment that only ever appears inside a larger, equally-or-
// more-duplicated fragment is redundant noise, so we drop the dominated spans and
// keep only maximal clones
fn _survivors(
    raw: Vec<Group>,
    spans: &[(usize, Occurrence)],
    config: &Config,
) -> Vec<Group> {
    let mut kept = Vec::new();
    for group in raw {
        if config.ignore.contains(&group.id) {
            continue;
        }
        let support = group.occurrences.len();
        let occurrences: Vec<Occurrence> = group
            .occurrences
            .into_iter()
            .filter(|occ| !_dominated(occ, support, spans))
            .collect();
        if occurrences.len() > 1 {
            kept.push(Group {
                id: group.id,
                holes: group.holes,
                occurrences,
            });
        }
    }
    kept
}

fn _dominated(occ: &Occurrence, support: usize, spans: &[(usize, Occurrence)]) -> bool {
    spans.iter().any(|(other_support, other)| {
        *other_support >= support && _contains(other, occ)
    })
}

fn _contains(outer: &Occurrence, inner: &Occurrence) -> bool {
    outer.path == inner.path
        && outer.start <= inner.start
        && outer.end >= inner.end
        && (outer.start < inner.start || outer.end > inner.end)
}
