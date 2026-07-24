pub fn common_prefix(modules: &[Vec<String>]) -> Vec<String> {
    let Some((first, rest)) = modules.split_first() else {
        return Vec::new();
    };
    let mut len = first.len();
    for module in rest {
        len = _shared(first, module, len);
    }
    first[..len].to_vec()
}

fn _shared(a: &[String], b: &[String], max: usize) -> usize {
    let mut count = 0;
    while count < max && count < b.len() && a[count] == b[count] {
        count += 1;
    }
    count
}
