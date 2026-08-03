use {
    super::main::ROOT,
    crate::t::FileConfig,
    std::{process, sync::LazyLock},
};

pub static FILE_CONFIG: LazyLock<FileConfig> = LazyLock::new(|| {
    let path = ROOT.join("enforcer.toml");
    let Ok(contents) = std::fs::read_to_string(&path) else {
        return FileConfig::default();
    };
    toml::from_str(&contents).unwrap_or_else(|e| {
        eprintln!("enforcer: invalid config in {}", path.display());
        eprintln!("{e}");
        process::exit(2);
    })
});
