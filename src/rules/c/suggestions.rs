use colored::Colorize;

const MAX_LISTED: usize = 3;

pub fn print(texts: &[String]) {
    for line in lines(texts) {
        println!("    {line}");
    }
}

pub fn lines(texts: &[String]) -> Vec<String> {
    match texts {
        [] => Vec::new(),
        [text] => vec![_dim(&format!("suggested fix: {text}"))],
        texts => _list(texts),
    }
}

pub fn shortlist(items: &[String]) -> String {
    let shown: Vec<&str> = items.iter().take(MAX_LISTED).map(String::as_str).collect();
    let more = items.len().saturating_sub(MAX_LISTED);
    let tail = if more > 0 {
        format!(" +{more} more")
    } else {
        String::new()
    };
    format!("{}{tail}", shown.join(", "))
}

fn _list(texts: &[String]) -> Vec<String> {
    let mut lines = vec![_dim("suggested fixes:")];
    lines.extend(texts.iter().map(|text| _dim(&format!("- {text}"))));
    lines
}

fn _dim(line: &str) -> String {
    line.dimmed().to_string()
}
