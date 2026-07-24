use syn::ItemFn;

use crate::checks::c::alpha;

pub fn canonical(item: &ItemFn) -> ItemFn {
    alpha::canonical_item_fn(item)
}
