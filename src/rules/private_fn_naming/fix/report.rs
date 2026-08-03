use {crate::s::FILE_CONFIG, colored::Colorize};

pub fn print(fixed: Vec<String>) {
    if fixed.is_empty() {
        _nothing_to_fix();
        return;
    }
    println!("{}", "fn-naming fix:".bold().cyan());
    println!("\n{}\n", "renamed private functions/methods in:".green());
    for line in &fixed {
        println!("  {line}");
    }
    println!();
}

fn _nothing_to_fix() {
    if FILE_CONFIG.debug {
        println!("{}", "fn-naming fix:".bold().cyan());
        println!("{} nothing to fix", "[success]".green().bold());
    }
}
