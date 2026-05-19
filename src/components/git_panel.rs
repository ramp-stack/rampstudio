// components/git_panel.rs — Git status panel.

use flowmango::{Canvas, GameObject};
use quartz::{tint_overlay, Arc, Align, Color, Font, Shared, Span, Text};

const COL_BG:      Color = Color(26,  27,  38,  255);
const COL_LABEL:   Color = Color(169, 177, 214, 255);
const COL_DIM:     Color = Color(86,  95,  137, 255);
const COL_ADDED:   Color = Color(158, 206, 106, 255);
const COL_MODIFIED:Color = Color(224, 175, 104, 255);
const COL_DELETED: Color = Color(247, 118, 142, 255);
const COL_STAGED:  Color = Color(122, 162, 247, 255);
const COL_SECTION: Color = Color(192, 202, 245, 255);

const FS:    f32 = 12.0;
const FS_SM: f32 = 11.0;
const PAD:   f32 = 12.0;
const ROW_H: f32 = 22.0;

#[derive(Clone, PartialEq)]
pub enum GitStatus { Added, Modified, Deleted, Staged }

#[derive(Clone)]
pub struct GitFile {
    pub path:   String,
    pub status: GitStatus,
}

#[derive(Clone)]
pub struct GitPanel {
    x: Shared<f32>,
    y: Shared<f32>,
    w: Shared<f32>,
    h: Shared<f32>,
    pub branch:        Shared<String>,
    pub staged:        Shared<Vec<GitFile>>,
    pub unstaged:      Shared<Vec<GitFile>>,
    font: Arc<Font>,
}

impl GitPanel {
    pub fn new(x: f32, y: f32, w: f32, h: f32, font: Arc<Font>) -> Self {
        Self {
            x: Shared::new(x), y: Shared::new(y),
            w: Shared::new(w), h: Shared::new(h),
            branch:   Shared::new(String::new()),
            staged:   Shared::new(Vec::new()),
            unstaged: Shared::new(Vec::new()),
            font,
        }
    }

    pub fn mount(&self, cv: &mut Canvas) {
        let (x, y, w, h) = self.bounds();
        cv.add_game_object("git_bg".into(),
            GameObject::build("git_bg").position(x, y).size(w, h).layer(4)
                .image(tint_overlay(w, h, COL_BG)).finish());

        self.spawn_label(cv, "git_header",  x + PAD, y + 10.0, "SOURCE CONTROL", FS_SM, COL_DIM);
        self.spawn_label(cv, "git_branch",  x + PAD, y + 32.0, "",               FS,    COL_STAGED);
        self.spawn_label(cv, "git_staged_hdr",   x + PAD, y + 60.0,  "STAGED CHANGES",   FS_SM, COL_SECTION);
        self.spawn_label(cv, "git_unstaged_hdr", x + PAD, y + 82.0,  "CHANGES",          FS_SM, COL_SECTION);

        for i in 0..30usize {
            let n = format!("git_staged_{i}");
            let mut o = GameObject::build(&n).position(x + PAD + 8.0, y).size(w, ROW_H).layer(5)
                .clip().clip_origin(x, y).clip_size(w, h).finish();
            o.set_drawable(Box::new(label_t("", FS_SM, COL_DIM, &self.font)));
            o.visible = false;
            cv.add_game_object(n, o);
        }
        for i in 0..30usize {
            let n = format!("git_unstaged_{i}");
            let mut o = GameObject::build(&n).position(x + PAD + 8.0, y).size(w, ROW_H).layer(5)
                .clip().clip_origin(x, y).clip_size(w, h).finish();
            o.set_drawable(Box::new(label_t("", FS_SM, COL_DIM, &self.font)));
            o.visible = false;
            cv.add_game_object(n, o);
        }
    }

    pub fn update(&self, cv: &mut Canvas) {
        let (x, y, w, h) = self.bounds();
        let branch   = self.branch.get().clone();
        let staged   = self.staged.get().clone();
        let unstaged = self.unstaged.get().clone();

        if let Some(o) = cv.get_game_object_mut("git_branch") {
            let b = if branch.is_empty() { "—".into() } else { format!(" {}", branch) };
            o.set_drawable(Box::new(label_t(&b, FS, COL_STAGED, &self.font)));
        }

        // Staged section
        let staged_hdr_y = y + 56.0;
        if let Some(o) = cv.get_game_object_mut("git_staged_hdr") {
            o.position = (x + PAD, staged_hdr_y);
            o.visible  = !staged.is_empty();
        }
        for i in 0..30usize {
            let n = format!("git_staged_{i}");
            let fy = staged_hdr_y + 20.0 + i as f32 * ROW_H;
            if let Some(o) = cv.get_game_object_mut(&n) {
                if let Some(f) = staged.get(i) {
                    let short = f.path.split('/').last().unwrap_or(&f.path);
                    let col   = status_color(&f.status);
                    let txt   = format!("  {} {}", status_letter(&f.status), short);
                    o.set_drawable(Box::new(label_t(&txt, FS_SM, col, &self.font)));
                    o.position = (x + PAD, fy);
                    o.visible  = true;
                } else { o.visible = false; }
            }
        }

        // Unstaged section
        let staged_h   = if staged.is_empty() { 0.0 } else { 20.0 + staged.len() as f32 * ROW_H };
        let unstaged_hdr_y = staged_hdr_y + staged_h + 8.0;
        if let Some(o) = cv.get_game_object_mut("git_unstaged_hdr") {
            o.position = (x + PAD, unstaged_hdr_y);
            o.visible  = !unstaged.is_empty();
        }
        for i in 0..30usize {
            let n = format!("git_unstaged_{i}");
            let fy = unstaged_hdr_y + 20.0 + i as f32 * ROW_H;
            if let Some(o) = cv.get_game_object_mut(&n) {
                if let Some(f) = unstaged.get(i) {
                    let short = f.path.split('/').last().unwrap_or(&f.path);
                    let col   = status_color(&f.status);
                    let txt   = format!("  {} {}", status_letter(&f.status), short);
                    o.set_drawable(Box::new(label_t(&txt, FS_SM, col, &self.font)));
                    o.position = (x + PAD, fy);
                    o.visible  = true;
                } else { o.visible = false; }
            }
        }

        // Background
        if let Some(o) = cv.get_game_object_mut("git_bg") {
            o.position = (x, y);
            if (o.size.0 - w).abs() > 0.5 || (o.size.1 - h).abs() > 0.5 {
                o.size = (w, h);
                o.set_image(tint_overlay(w, h, COL_BG));
            }
        }
    }

    pub fn resize(&self, cv: &mut Canvas, x: f32, y: f32, w: f32, h: f32) {
        *self.x.get_mut() = x; *self.y.get_mut() = y;
        *self.w.get_mut() = w; *self.h.get_mut() = h;
        self.update(cv);
    }

    pub fn show(&self, cv: &mut Canvas) {
        for n in &["git_bg","git_header","git_branch"] {
            if let Some(o) = cv.get_game_object_mut(n) { o.visible = true; }
        }
    }
    pub fn hide(&self, cv: &mut Canvas) {
        for n in &["git_bg","git_header","git_branch","git_staged_hdr","git_unstaged_hdr"] {
            if let Some(o) = cv.get_game_object_mut(n) { o.visible = false; }
        }
        for i in 0..30usize {
            for pfx in &["git_staged_","git_unstaged_"] {
                let n = format!("{pfx}{i}");
                if let Some(o) = cv.get_game_object_mut(&n) { o.visible = false; }
            }
        }
    }

    /// Refresh from the filesystem using `git status`.
    pub fn refresh(&self, project_root: &str) {
        use std::process::Command;
        let out = Command::new("git").args(["status","--porcelain"])
            .current_dir(project_root).output();
        let Ok(out) = out else { return; };
        let text = String::from_utf8_lossy(&out.stdout);
        let mut staged   = Vec::new();
        let mut unstaged = Vec::new();
        for line in text.lines() {
            if line.len() < 3 { continue; }
            let xy   = &line[..2];
            let path = line[3..].to_string();
            let x_ch = xy.chars().next().unwrap_or(' ');
            let y_ch = xy.chars().nth(1).unwrap_or(' ');
            if x_ch != ' ' && x_ch != '?' {
                staged.push(GitFile { path: path.clone(), status: char_to_status(x_ch) });
            }
            if y_ch != ' ' {
                unstaged.push(GitFile { path, status: char_to_status(y_ch) });
            }
        }
        *self.staged.get_mut()   = staged;
        *self.unstaged.get_mut() = unstaged;
    }

    fn bounds(&self) -> (f32, f32, f32, f32) {
        (*self.x.get(), *self.y.get(), *self.w.get(), *self.h.get())
    }

    fn spawn_label(&self, cv: &mut Canvas, key: &str, x: f32, y: f32, text: &str, fs: f32, col: Color) {
        let mut o = GameObject::build(key).position(x, y).size(200.0, fs * 1.4).layer(5).finish();
        o.set_drawable(Box::new(label_t(text, fs, col, &self.font)));
        cv.add_game_object(key.into(), o);
    }
}

fn char_to_status(c: char) -> GitStatus {
    match c { 'A' => GitStatus::Added, 'D' => GitStatus::Deleted,
              '?' => GitStatus::Added, _   => GitStatus::Modified }
}
fn status_color(s: &GitStatus) -> Color {
    match s { GitStatus::Added => COL_ADDED, GitStatus::Modified => COL_MODIFIED,
              GitStatus::Deleted => COL_DELETED, GitStatus::Staged => COL_STAGED }
}
fn status_letter(s: &GitStatus) -> &'static str {
    match s { GitStatus::Added => "A", GitStatus::Modified => "M",
              GitStatus::Deleted => "D", GitStatus::Staged => "S" }
}
fn label_t(s: &str, fs: f32, color: Color, font: &Arc<Font>) -> Text {
    Text::new(vec![Span::new(s.to_string(), fs, Some(fs * 1.4), font.clone(), color, 0.0)],
        None, Align::Left, None)
}