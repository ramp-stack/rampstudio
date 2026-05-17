pub fn resolve_project_root() -> String {
    let raw = std::env::var("RAMP_PROJECT")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| ".".to_string());
    match std::fs::canonicalize(&raw) {
        Ok(p)  => p.to_string_lossy().into_owned(),
        Err(_) => raw,
    }
}

pub fn pick_initial_file(root: &str) -> Option<String> {
    const CANDIDATES: &[&str] = &[
        "src/main.rs", "src/lib.rs", "main.rs", "lib.rs",
        "README.md",   "readme.md",  "index.js", "index.ts",
    ];
    for c in CANDIDATES {
        let p = std::path::Path::new(root).join(c);
        if p.is_file() {
            return Some(p.to_string_lossy().into_owned());
        }
    }
    std::fs::read_dir(root).ok()?.flatten()
        .find(|e| e.path().is_file())
        .map(|e| e.path().to_string_lossy().into_owned())
}