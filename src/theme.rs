// src/theme.rs — Single theme.json controls chrome, explorer, syntax, and ui.

pub struct ChromeTheme {
    pub app_bg:              [u8; 4],
    pub topbar_bg:           [u8; 4],
    pub sidebar_bg:          [u8; 4],
    pub border:              [u8; 4],
    pub sidebar_icon:        [u8; 4],
    pub sidebar_icon_active: [u8; 4],
    pub statusbar_bg:        [u8; 4],
    pub statusbar_fg:        [u8; 4],
    pub statusbar_hover:     [u8; 4],
}

impl Default for ChromeTheme {
    fn default() -> Self {
        Self {
            app_bg:              [0,   0,   0,   255],
            topbar_bg:           [13,  13,  13,  255],
            sidebar_bg:          [13,  13,  13,  255],
            border:              [255, 255, 255, 20 ],
            sidebar_icon:        [255, 255, 255, 90 ],
            sidebar_icon_active: [255, 255, 255, 255],
            statusbar_bg:        [0,   0,   0,   255],
            statusbar_fg:        [180, 180, 180, 255],
            statusbar_hover:     [255, 255, 255, 18 ],
        }
    }
}

pub struct ExplorerTheme {
    pub bg:          String,
    pub text:        String,
    pub file:        String,
    pub crumb:       String,
    pub folder_icon: String,
    pub guide:       String,
}

impl Default for ExplorerTheme {
    fn default() -> Self {
        Self {
            bg:          "#0d0d0d".into(),
            text:        "#cccccc".into(),
            file:        "#7a7f8a".into(),
            crumb:       "#3a3a4a".into(),
            folder_icon: "#7c6fcd".into(),
            guide:       "#1e1e2e".into(),
        }
    }
}

pub struct AppTheme {
    pub chrome:   ChromeTheme,
    pub explorer: ExplorerTheme,
    /// Full bytes passed to EditorComponent — editor reads "syntax" + "ui" from this.
    pub raw_bytes: Vec<u8>,
}

impl AppTheme {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let chrome   = parse_chrome(&bytes).unwrap_or_default();
        let explorer = parse_explorer(&bytes).unwrap_or_default();
        Self { chrome, explorer, raw_bytes: bytes }
    }
}

// ── Parser ────────────────────────────────────────────────────────────────────

fn parse_chrome(bytes: &[u8]) -> Option<ChromeTheme> {
    let s = std::str::from_utf8(bytes).ok()?;
    let b = extract_object(s, "chrome")?;
    Some(ChromeTheme {
        app_bg:              hex_rgba(read_str(&b, "app_bg")?)?,
        topbar_bg:           hex_rgba(read_str(&b, "topbar_bg")?)?,
        sidebar_bg:          hex_rgba(read_str(&b, "sidebar_bg")?)?,
        border:              hex_rgba(read_str(&b, "border")?)?,
        sidebar_icon:        hex_rgba(read_str(&b, "sidebar_icon")?)?,
        sidebar_icon_active: hex_rgba(read_str(&b, "sidebar_icon_active")?)?,
        statusbar_bg:        hex_rgba(read_str(&b, "statusbar_bg")?)?,
        statusbar_fg:        hex_rgba(read_str(&b, "statusbar_fg")?)?,
        statusbar_hover:     hex_rgba(read_str(&b, "statusbar_hover")?)?,
    })
}

fn parse_explorer(bytes: &[u8]) -> Option<ExplorerTheme> {
    let s = std::str::from_utf8(bytes).ok()?;
    let b = extract_object(s, "explorer")?;
    Some(ExplorerTheme {
        bg:          read_str(&b, "bg")?.to_string(),
        text:        read_str(&b, "text")?.to_string(),
        file:        read_str(&b, "file")?.to_string(),
        crumb:       read_str(&b, "crumb")?.to_string(),
        folder_icon: read_str(&b, "folder_icon")?.to_string(),
        guide:       read_str(&b, "guide")?.to_string(),
    })
}

fn extract_object(json: &str, key: &str) -> Option<String> {
    let needle = format!("\"{}\"", key);
    let start  = json.find(&needle)?;
    let after  = &json[start + needle.len()..];
    let colon  = after.find(':')? + 1;
    let body   = after[colon..].trim_start();
    if !body.starts_with('{') { return None; }
    let mut depth = 0usize;
    let mut end   = 0usize;
    for (i, ch) in body.char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => { depth -= 1; if depth == 0 { end = i + 1; break; } }
            _   => {}
        }
    }
    Some(body[..end].to_string())
}

fn read_str<'a>(obj: &'a str, key: &str) -> Option<&'a str> {
    let needle = format!("\"{}\"", key);
    let start  = obj.find(&needle)?;
    let after  = &obj[start + needle.len()..];
    let colon  = after.find(':')? + 1;
    let val    = after[colon..].trim_start();
    if !val.starts_with('"') { return None; }
    let inner  = &val[1..];
    let end    = inner.find('"')?;
    Some(&inner[..end])
}

pub fn hex_rgba(hex: &str) -> Option<[u8; 4]> {
    let h = hex.trim_start_matches('#');
    match h.len() {
        6 => Some([
            u8::from_str_radix(&h[0..2], 16).ok()?,
            u8::from_str_radix(&h[2..4], 16).ok()?,
            u8::from_str_radix(&h[4..6], 16).ok()?,
            255,
        ]),
        8 => Some([
            u8::from_str_radix(&h[0..2], 16).ok()?,
            u8::from_str_radix(&h[2..4], 16).ok()?,
            u8::from_str_radix(&h[4..6], 16).ok()?,
            u8::from_str_radix(&h[6..8], 16).ok()?,
        ]),
        _ => None,
    }
}