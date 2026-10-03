use {
    super::{
        folders, index,
        t::{FileImports, Folder, Member},
        verdict,
    },
    crate::rules::{
        c::path,
        t::{FileViolation, PartReport},
    },
    colored::Colorize,
};

pub fn part() -> PartReport {
    let imports = index::build();
    let mut passed = 0;
    let mut violations = Vec::new();
    for folder in folders::all() {
        let unneeded = verdict::unneeded(&folder, &imports);
        passed += folder.members.len() - unneeded.len();
        for member in &folder.members {
            if unneeded.contains(member.name.as_str()) {
                violations.push(_render(&folder, member, &imports));
            }
        }
    }
    PartReport {
        name: "c-common",
        failures_header: "The following module(s) sit in a `c` common module, but are only imported from beside it, where a sideways import would do:",
        unit: "c modules",
        passed,
        violations,
    }
}

fn _render(folder: &Folder, member: &Member, imports: &[FileImports]) -> FileViolation {
    let mut lines = vec!["imported only by:".red().to_string()];
    for importer in verdict::importers(folder, member, imports) {
        lines.push(format!("  {}", path::rel(&importer.path)));
    }
    let hint = format!(
        "move it up beside them, to {}, declared with a private `mod`",
        _destination(member)
    );
    lines.push(hint.dimmed().to_string());
    FileViolation {
        path: path::rel(&member.path),
        lines,
    }
}

fn _destination(member: &Member) -> String {
    let parent = member.path.parent().and_then(|c| c.parent());
    match (parent, member.path.file_name()) {
        (Some(parent), Some(file_name)) => path::rel(&parent.join(file_name)),
        _ => member.name.clone(),
    }
}
