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
    "line_height_mul":          {:.3},
    "char_width_mul":           {:.4},
    "text_x":                   {:.1},
    "text_y":                   {:.1},
    "gutter_w":                 {:.1},
    "backspace_deletes_before": {},
    "cursor_blink":             {},
    "auto_pairs":               {},
    "border_thickness":         {:.1},
    "border_padding":           {:.1},
    "scroll_accel":             {:.2},
    "scroll_friction":          {:.3},
    "scroll_max":               {:.1}
  }}"#,
        s.font_size,
        s.line_height_mul,
        s.char_width_mul,
        s.text_x,
        s.text_y,
        s.gutter_w,
        s.backspace_deletes_before,
        s.cursor_blink,
        s.auto_pairs,
        s.border_thickness,
        s.border_padding,
        s.scroll_accel,
        s.scroll_friction,
        s.scroll_max,
    )
}

fn parse_editor(t: &str, s: &mut EditorSettings) {
    if let Some(v) = get_f(t, "font_size") {
        s.font_size = v;
    }
    if let Some(v) = get_f(t, "line_height_mul") {
        s.line_height_mul = v;
    }
    if let Some(v) = get_f(t, "char_width_mul") {
        s.char_width_mul = v;
    }
    if let Some(v) = get_f(t, "text_x") {
        s.text_x = v;
    }
    if let Some(v) = get_f(t, "text_y") {
        s.text_y = v;
    }
    if let Some(v) = get_f(t, "gutter_w") {
        s.gutter_w = v;
    }
    if let Some(v) = get_b(t, "backspace_deletes_before") {
        s.backspace_deletes_before = v;
    }
    if let Some(v) = get_b(t, "cursor_blink") {
        s.cursor_blink = v;
    }
    if let Some(v) = get_b(t, "auto_pairs") {
        s.auto_pairs = v;
    }
    if let Some(v) = get_f(t, "border_thickness") {
        s.border_thickness = v;
    }
    if let Some(v) = get_f(t, "border_padding") {
        s.border_padding = v;
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
    "folder_label_font_size":              {:.1},
    "file_label_font_size":                     {:.1},
  }}"#,
        s.folder_label_font_size, s.file_label_font_size,
    )
}

fn parse_explorer(t: &str, s: &mut ExplorerSettings) {
    if let Some(v) = get_f(t, "folder_label_font_size") {
        s.folder_label_font_size = v;
    }
    if let Some(v) = get_f(t, "file_label_size") {
        s.file_label_font_size = v;
    }
}

// ── Terminal ──────────────────────────────────────────────────────────────────

fn serialize_terminal(s: &TermSettings) -> String {
    format!(
        r#"  "terminal": {{
    "font_size":   {:.1},
    "line_height": {:.3},
    "pad_x":       {:.1},
    "pad_y":       {:.1},
    "scrollback":  {},
    "col_text":    "{}",
    "col_prompt":  "{}",
    "col_input":   "{}",
    "col_cursor":  "{}",
    "col_error":   "{}"
  }}"#,
        s.font_size,
        s.line_height,
        s.pad_x,
        s.pad_y,
        s.scrollback,
        color_to_hex(s.col_text),
        color_to_hex(s.col_prompt),
        color_to_hex(s.col_input),
        color_to_hex(s.col_cursor),
        color_to_hex(s.col_error),
    )
}

fn parse_terminal(t: &str, s: &mut TermSettings) {
    if let Some(v) = get_f(t, "font_size") {
        s.font_size = v;
    }
    if let Some(v) = get_f(t, "line_height") {
        s.line_height = v;
    }
    if let Some(v) = get_f(t, "pad_x") {
        s.pad_x = v;
    }
    if let Some(v) = get_f(t, "pad_y") {
        s.pad_y = v;
    }
    if let Some(v) = get_us(t, "scrollback") {
        s.scrollback = v;
    }
    if let Some(v) = get_col(t, "col_text") {
        s.col_text = v;
    }
    if let Some(v) = get_col(t, "col_prompt") {
        s.col_prompt = v;
    }
    if let Some(v) = get_col(t, "col_input") {
        s.col_input = v;
    }
    if let Some(v) = get_col(t, "col_cursor") {
        s.col_cursor = v;
    }
    if let Some(v) = get_col(t, "col_error") {
        s.col_error = v;
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

pub fn ensure_file() {
    if !std::path::Path::new(SETTINGS_FILENAME).exists() {
        let mut ed = EditorSettings::default();
        ed.backspace_deletes_before = true;
        ed.auto_pairs = true;
        save(&ed, &ExplorerSettings::default(), &TermSettings::default());
    }
}
