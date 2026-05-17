use std::sync::{Arc, Mutex};
use std::process::Command;

#[derive(Clone)]
pub struct GitInfo {
    pub branch:      Arc<Mutex<String>>,
    pub last_commit: Arc<Mutex<String>>,
}

impl GitInfo {
    pub fn new() -> Self {
        Self {
            branch:      Arc::new(Mutex::new(String::new())),
            last_commit: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn start_polling(&self, project_root: String, interval_secs: u64) {
        let branch      = self.branch.clone();
        let last_commit = self.last_commit.clone();

        std::thread::spawn(move || {
            loop {
                let br = run_git(&project_root, &["rev-parse", "--abbrev-ref", "HEAD"])
                    .unwrap_or_else(|| "—".into());
                if let Ok(mut g) = branch.lock() { *g = br; }

                let lc = run_git(&project_root, &["log", "-1", "--pretty=format:%an (%cr)"])
                    .unwrap_or_else(|| "—".into());
                if let Ok(mut g) = last_commit.lock() { *g = lc; }

                std::thread::sleep(std::time::Duration::from_secs(interval_secs));
            }
        });
    }

    pub fn read_branch(&self)      -> String { self.branch.lock().map(|g| g.clone()).unwrap_or_default() }
    pub fn read_last_commit(&self) -> String { self.last_commit.lock().map(|g| g.clone()).unwrap_or_default() }
}

fn run_git(cwd: &str, args: &[&str]) -> Option<String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .ok()?;
    if !out.status.success() { return None; }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() { None } else { Some(s) }
}

pub fn lang_for_path(path: &str) -> &'static str {
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    match ext.as_str() {
        "rs"                    => "Rust",
        "js"                    => "JavaScript",
        "jsx"                   => "JSX",
        "ts"                    => "TypeScript",
        "tsx"                   => "TSX",
        "py"                    => "Python",
        "html" | "htm"          => "HTML",
        "css"                   => "CSS",
        "json"                  => "JSON",
        "toml"                  => "TOML",
        "yaml" | "yml"          => "YAML",
        "md"                    => "Markdown",
        "sh" | "bash" | "zsh"   => "Shell",
        "c"                     => "C",
        "cpp" | "cc" | "cxx"    => "C++",
        "h" | "hpp"             => "C/C++ Header",
        "go"                    => "Go",
        "java"                  => "Java",
        "kt"                    => "Kotlin",
        "swift"                 => "Swift",
        "rb"                    => "Ruby",
        "php"                   => "PHP",
        "lua"                   => "Lua",
        "r"                     => "R",
        "sql"                   => "SQL",
        "xml"                   => "XML",
        "txt"                   => "Plain Text",
        "lock"                  => "Lock File",
        _                       => "Text",
    }
}