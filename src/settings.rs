use crate::preferences::SETTINGS_FILENAME;
use editor::prelude::Settings as EditorSettings;
use explorer::ExplorerSettings;
use quartz::Color;
use terminal::preferences::TermSettings;

fn color_to_hex(c: Color) -> String {
    format!("#{:02X}{:02X}{:02X}", c.0, c.1, c.2)
}

fn parse_hex_color(hex: &str) -> Option<Color> {
    let s = hex.trim().trim_matches('"').trim_start_matches('#');
    let v = u32::from_str_radix(s, 16).ok()?;
    Some(Color(
        ((v >> 16) & 0xFF) as u8,
        ((v >> 8) & 0xFF) as u8,
        (v & 0xFF) as u8,
        255,
    ))
}

fn find_val<'a>(src: &'a str, key: &str) -> Option<&'a str> {
    let needle = format!("\"{}\"", key);
    let i = src.find(&needle)?;
    let rest = &src[i + needle.len()..];
    let after = rest[rest.find(':')? + 1..].trim_start();
    let end = after
        .find(|c: char| c == ',' || c == '}' || c == '\n')
        .unwrap_or(after.len());
    Some(after[..end].trim())
}

fn get_f(s: &str, k: &str) -> Option<f32> {
    find_val(s, k)?.parse().ok()
}
fn get_b(s: &str, k: &str) -> Option<bool> {
    find_val(s, k)?.parse().ok()
}
fn get_us(s: &str, k: &str) -> Option<usize> {
    find_val(s, k)?.parse().ok()
}
fn get_col(s: &str, k: &str) -> Option<Color> {
    parse_hex_color(find_val(s, k)?)
}

// ── Editor ────────────────────────────────────────────────────────────────────

fn serialize_editor(s: &EditorSettings) -> String {
    format!(
        r#"  "editor": {{
    "font_size":                {:.2},
    "scroll_accel":             {:.2},
    "scroll_friction":          {:.3},
    "scroll_max":               {:.1}
  }}"#,
        s.font_size, s.scroll_accel, s.scroll_friction, s.scroll_max,
    )
}

fn parse_editor(t: &str, s: &mut EditorSettings) {
    if let Some(v) = get_f(t, "font_size") {
        s.font_size = v;
    }
    if let Some(v) = get_f(t, "scroll_accel") {
        s.scroll_accel = v;
    }
    if let Some(v) = get_f(t, "scroll_friction") {
        s.scroll_friction = v;
    }
    if let Some(v) = get_f(t, "scroll_max") {
        s.scroll_max = v;
    }
}

// ── Explorer ──────────────────────────────────────────────────────────────────

fn serialize_explorer(s: &ExplorerSettings) -> String {
    format!(
        r#"  "explorer": {{
    "font_size":                     {:.1},
  }}"#,
        s.font_size,
    )
}

fn parse_explorer(t: &str, s: &mut ExplorerSettings) {
    if let Some(v) = get_f(t, "font_size") {
        s.font_size = v;
    }
}

// ── Terminal ──────────────────────────────────────────────────────────────────

fn serialize_terminal(s: &TermSettings) -> String {
    format!(
        r#"  "terminal": {{
    "font_size":   {:.1},
  }}"#,
        s.font_size,
    )
}

fn parse_terminal(t: &str, s: &mut TermSettings) {
    if let Some(v) = get_f(t, "font_size") {
        s.font_size = v;
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

pub fn save(ed: &EditorSettings, ex: &ExplorerSettings, term: &TermSettings) {
    let json = format!(
        "{{\n{},\n{},\n{}\n}}\n",
        serialize_editor(ed),
        serialize_explorer(ex),
        serialize_terminal(term)
    );
    let _ = std::fs::write(SETTINGS_FILENAME, json);
}

pub fn load(ed: &mut EditorSettings, ex: &mut ExplorerSettings, term: &mut TermSettings) {
    if let Ok(txt) = std::fs::read_to_string(SETTINGS_FILENAME) {
        parse_editor(&txt, ed);
        parse_explorer(&txt, ex);
        parse_terminal(&txt, term);
    }
}

// Creates SETTINGS_FILENAME if it doesn't exist.
pub fn ensure_file() {
    if !std::path::Path::new(SETTINGS_FILENAME).exists() {
        let mut ed = EditorSettings::default();
        ed.backspace_deletes_before = true;
        ed.auto_pairs = true;
        save(&ed, &ExplorerSettings::default(), &TermSettings::default());
    }
}
