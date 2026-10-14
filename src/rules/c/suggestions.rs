use colored::Colorize;

const MAX_LISTED: usize = 3;

pub fn print(texts: &[String]) {
    match texts {
        [] => {}
        [text] => _line(&format!("suggested fix: {text}")),
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

fn _list(texts: &[String]) {
    _line("suggested fixes:");
    for text in texts {
        _line(&format!("- {text}"));
    }
}

fn _line(line: &str) {
    println!("    {}", line.dimmed());
}
