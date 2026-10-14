use {
    super::{
        cluster, folders,
        t::{Folder, Member},
        verdict,
    },
    crate::rules::{
        c::{graph, path},
        t::{FileImports, FileViolation, PartReport},
    },
    colored::Colorize,
};

pub fn part() -> PartReport {
    let imports = graph::build();
    let mut passed = 0;
    let mut violations = Vec::new();
    for folder in folders::all() {
        let unneeded = verdict::unneeded(&folder, &imports);
        passed += folder.members.len() - unneeded.len();
        for members in cluster::group(&folder, &unneeded, &imports) {
            violations.push(_render(&folder, &members, &imports));
        }
    }
    PartReport {
        name: "c-common",
        failures_header: "The following module(s) sit in a `c` common module, but are only imported from beside it, where a sideways import would do:",
        unit: "c module groups",
        passed,
        violations,
    }
}

fn _render(folder: &Folder, members: &[&Member], imports: &[FileImports]) -> FileViolation {
    let names: Vec<&str> = members.iter().map(|m| m.name.as_str()).collect();
    let mut lines = vec![
        "only used from beside this `c`:".red().to_string(),
        format!("  {}", names.join(", ")),
        "imported only by:".red().to_string(),
    ];
    for importer in cluster::importers(folder, members, imports) {
        lines.push(format!("  {}", path::rel(&importer.path)));
    }
    let hint = "move out of `c`, beside the importers, or under the importer if there is only one";
    lines.push(hint.dimmed().to_string());
    FileViolation {
        path: path::rel(_location(folder, members)),
        lines,
    }
}

fn _location<'a>(folder: &'a Folder, members: &[&'a Member]) -> &'a std::path::Path {
    match members {
        [member] => &member.path,
        _ => &folder.path,
    }
}
