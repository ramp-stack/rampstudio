use quartz::Color;
use editor::prelude::Settings as EditorSettings;
use explorer::ExplorerSettings;
use terminal::preferences::TermSettings;
use crate::preferences::SETTINGS_JSON_PATH;

// ── Colour helpers ────────────────────────────────────────────────────────────

pub fn color_to_hex(c: Color) -> String {
    format!("#{:02X}{:02X}{:02X}", c.0, c.1, c.2)
}

pub fn parse_hex_color(hex: &str) -> Option<Color> {
    let s = hex.trim().trim_matches('"').trim_start_matches('#');
    let v = u32::from_str_radix(s, 16).ok()?;
    Some(Color(
        ((v >> 16) & 0xFF) as u8,
        ((v >> 8)  & 0xFF) as u8,
        (v         & 0xFF) as u8,
        255,
    ))
}

// ── Key extraction ────────────────────────────────────────────────────────────

pub fn find_val<'a>(src: &'a str, key: &str) -> Option<&'a str> {
    let needle = format!("\"{}\"", key);
    let i = src.find(&needle)?;
    let rest = &src[i + needle.len()..];
    let colon = rest.find(':')?;
    let after = rest[colon + 1..].trim_start();
    let end = after.find(|c: char| c == ',' || c == '}' || c == '\n')
        .unwrap_or(after.len());
    Some(after[..end].trim())
}

pub fn get_f(src: &str, k: &str)   -> Option<f32>   { find_val(src, k)?.parse().ok() }
pub fn get_b(src: &str, k: &str)   -> Option<bool>  { find_val(src, k)?.parse().ok() }
pub fn get_us(src: &str, k: &str)  -> Option<usize> { find_val(src, k)?.parse().ok() }
pub fn get_col(src: &str, k: &str) -> Option<Color> { parse_hex_color(find_val(src, k)?) }

// ── Editor ────────────────────────────────────────────────────────────────────

pub fn serialize_editor(s: &EditorSettings) -> String {
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
        s.font_size, s.line_height_mul, s.char_width_mul,
        s.text_x, s.text_y, s.gutter_w,
        s.backspace_deletes_before, s.cursor_blink, s.auto_pairs,
        s.border_thickness, s.border_padding,
        s.scroll_accel, s.scroll_friction, s.scroll_max,
    )
}

pub fn parse_editor_into(text: &str, s: &mut EditorSettings) {
    if let Some(v) = get_f(text, "font_size")                { s.font_size = v; }
    if let Some(v) = get_f(text, "line_height_mul")          { s.line_height_mul = v; }
    if let Some(v) = get_f(text, "char_width_mul")           { s.char_width_mul = v; }
    if let Some(v) = get_f(text, "text_x")                   { s.text_x = v; }
    if let Some(v) = get_f(text, "text_y")                   { s.text_y = v; }
    if let Some(v) = get_f(text, "gutter_w")                 { s.gutter_w = v; }
    if let Some(v) = get_b(text, "backspace_deletes_before") { s.backspace_deletes_before = v; }
    if let Some(v) = get_b(text, "cursor_blink")             { s.cursor_blink = v; }
    if let Some(v) = get_b(text, "auto_pairs")               { s.auto_pairs = v; }
    if let Some(v) = get_f(text, "border_thickness")         { s.border_thickness = v; }
    if let Some(v) = get_f(text, "border_padding")           { s.border_padding = v; }
    if let Some(v) = get_f(text, "scroll_accel")             { s.scroll_accel = v; }
    if let Some(v) = get_f(text, "scroll_friction")          { s.scroll_friction = v; }
    if let Some(v) = get_f(text, "scroll_max")               { s.scroll_max = v; }
}

// ── Explorer ──────────────────────────────────────────────────────────────────

pub fn serialize_explorer(s: &ExplorerSettings) -> String {
    format!(
r#"  "explorer": {{
    "row_height":       {:.1},
    "indent":           {:.1},
    "pad_left":         {:.1},
    "arrow_pad":        {:.1},
    "arrow_size":       {:.1},
    "chevron_w":        {:.1},
    "min_width":        {:.1},
    "max_depth":        {},
    "font_size":        {:.1},
    "file_size":        {:.1},
    "char_w_folder":    {:.2},
    "char_w_file":      {:.2},
    "scroll_speed":     {:.2},
    "scroll_speed_max": {:.1},
    "max_slots":        {}
  }}"#,
        s.row_height, s.indent, s.pad_left,
        s.arrow_pad, s.arrow_size, s.chevron_w,
        s.min_width, s.max_depth,
        s.font_size, s.file_size,
        s.char_w_folder, s.char_w_file,
        s.scroll_speed, s.scroll_speed_max,
        s.max_slots,
    )
}

pub fn parse_explorer_into(text: &str, s: &mut ExplorerSettings) {
    if let Some(v) = get_f(text, "row_height")       { s.row_height = v; }
    if let Some(v) = get_f(text, "indent")           { s.indent = v; }
    if let Some(v) = get_f(text, "pad_left")         { s.pad_left = v; }
    if let Some(v) = get_f(text, "arrow_pad")        { s.arrow_pad = v; }
    if let Some(v) = get_f(text, "arrow_size")       { s.arrow_size = v; }
    if let Some(v) = get_f(text, "chevron_w")        { s.chevron_w = v; }
    if let Some(v) = get_f(text, "min_width")        { s.min_width = v; }
    if let Some(v) = get_us(text, "max_depth")       { s.max_depth = v; }
    if let Some(v) = get_f(text, "font_size")        { s.font_size = v; }
    if let Some(v) = get_f(text, "file_size")        { s.file_size = v; }
    if let Some(v) = get_f(text, "char_w_folder")    { s.char_w_folder = v; }
    if let Some(v) = get_f(text, "char_w_file")      { s.char_w_file = v; }
    if let Some(v) = get_f(text, "scroll_speed")     { s.scroll_speed = v; }
    if let Some(v) = get_f(text, "scroll_speed_max") { s.scroll_speed_max = v; }
    if let Some(v) = get_us(text, "max_slots")       { s.max_slots = v; }
}

// ── Terminal ──────────────────────────────────────────────────────────────────

pub fn serialize_terminal(s: &TermSettings) -> String {
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
        s.font_size, s.line_height, s.pad_x, s.pad_y,
        s.scrollback,
        color_to_hex(s.col_text),
        color_to_hex(s.col_prompt),
        color_to_hex(s.col_input),
        color_to_hex(s.col_cursor),
        color_to_hex(s.col_error),
    )
}

pub fn parse_terminal_into(text: &str, s: &mut TermSettings) {
    if let Some(v) = get_f(text,   "font_size")   { s.font_size   = v; }
    if let Some(v) = get_f(text,   "line_height") { s.line_height = v; }
    if let Some(v) = get_f(text,   "pad_x")       { s.pad_x       = v; }
    if let Some(v) = get_f(text,   "pad_y")       { s.pad_y       = v; }
    if let Some(v) = get_us(text,  "scrollback")  { s.scrollback  = v; }
    if let Some(v) = get_col(text, "col_text")    { s.col_text    = v; }
    if let Some(v) = get_col(text, "col_prompt")  { s.col_prompt  = v; }
    if let Some(v) = get_col(text, "col_input")   { s.col_input   = v; }
    if let Some(v) = get_col(text, "col_cursor")  { s.col_cursor  = v; }
    if let Some(v) = get_col(text, "col_error")   { s.col_error   = v; }
}

// ── Unified file ──────────────────────────────────────────────────────────────

pub fn save(ed: &EditorSettings, ex: &ExplorerSettings, term: &TermSettings) {
    let json = format!(
        "{{\n{},\n{},\n{}\n}}\n",
        serialize_editor(ed),
        serialize_explorer(ex),
        serialize_terminal(term),
    );
    let _ = std::fs::write(SETTINGS_JSON_PATH, json);
}

pub fn load(
    ed:   &mut EditorSettings,
    ex:   &mut ExplorerSettings,
    term: &mut TermSettings,
) {
    if let Ok(txt) = std::fs::read_to_string(SETTINGS_JSON_PATH) {
        parse_editor_into(&txt, ed);
        parse_explorer_into(&txt, ex);
        parse_terminal_into(&txt, term);
    }
}

pub fn ensure_file() {
    if !std::path::Path::new(SETTINGS_JSON_PATH).exists() {
        let mut ed = EditorSettings::default();
        ed.backspace_deletes_before = true;
        ed.auto_pairs               = true;
        let ex   = ExplorerSettings::default();
        let term = TermSettings::default();
        save(&ed, &ex, &term);
    }
}