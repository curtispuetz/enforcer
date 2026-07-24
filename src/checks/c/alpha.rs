use crate::checks::t::AlphaCollector;
use crate::checks::t::AlphaRenamer;

pub fn canonicalize<N>(
    node: &mut N,
    collect: impl Fn(&mut AlphaCollector, &N),
    rename: impl Fn(&mut AlphaRenamer, &mut N),
) {
    let mut collector = AlphaCollector::default();
    collect(&mut collector, node);
    let mut renamer = AlphaRenamer {
        values: collector.values,
        lifetimes: collector.lifetimes,
    };
    rename(&mut renamer, node);
}
