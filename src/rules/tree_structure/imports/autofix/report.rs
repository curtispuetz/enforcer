use {crate::s::FILE_CONFIG, colored::Colorize};

pub fn print(fixed: Vec<String>, manual: usize) {
    if fixed.is_empty() && manual == 0 && !FILE_CONFIG.debug {
        return;
    }
    println!("{}", "tree-structure fix:".bold().cyan());
    if fixed.is_empty() {
        println!("{} no imports to rewrite", "[success]".green().bold());
    } else {
        println!("\n{}\n", "rewrote imports in:".green());
        for line in &fixed {
            println!("  {line}");
        }
    }
    if manual > 0 {
        let left = format!(
            "{manual} import(s) still need moving to a reachable common module by \
            hand (`cargo enforcer tree-structure` lists them)"
        );
        println!("\n{}", left.yellow());
    }
    println!();
}
