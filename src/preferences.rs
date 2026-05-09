use quartz::Color;

// ── App colours ───────────────────────────────────────────────────────────────
pub const COL_APP_BG:     Color = Color(0, 0, 0, 255);
pub const COL_TOPBAR_BG:  Color = Color(0, 0, 0, 255);
pub const COL_BORDER:     Color = Color(255, 255, 255, 80);
pub const COL_SIDEBAR_BG: Color = Color(0, 0, 0, 255);

// ── Chrome dimensions ─────────────────────────────────────────────────────────
pub const TOPBAR_H: f32 = 44.0;

// ── Left activity sidebar ─────────────────────────────────────────────────────
pub const SIDEBAR_W:                 f32   = 48.0;
pub const SIDEBAR_ICON_SIZE:         f32   = 28.0;
pub const SIDEBAR_ICON_TOP:          f32   = 20.0;
pub const SIDEBAR_ICON_GAP:          f32   = 32.0;
pub const SIDEBAR_ICON_COLOR:        Color = Color(255, 255, 255, 90);
pub const SIDEBAR_ICON_COLOR_ACTIVE: Color = Color(255, 255, 255, 255);

// sidebar_active canvas var values:
//   0 = nothing active (explorer hidden)
//   1 = files icon active (explorer visible)
pub const SIDEBAR_NONE:  u8 = 0;
pub const SIDEBAR_FILES: u8 = 1;

// Phosphor Bold codepoints
pub const PH_FILES:      &str = "\u{e710}";
pub const PH_SEARCH:     &str = "\u{e30c}";
pub const PH_USERS:      &str = "\u{e4d6}";
pub const PH_TERMINAL:   &str = "\u{e47e}";
pub const PH_GIT:        &str = "\u{e278}";
pub const PH_EXTENSIONS: &str = "\u{e596}";

// ── Layout constants ──────────────────────────────────────────────────────────
pub const INIT_EXPLORER_RATIO: f32 = 0.18;
pub const INIT_TERMINAL_RATIO: f32 = 0.28;
pub const INIT_CW:             f32 = 1920.0;
pub const INIT_CH:             f32 = 1080.0;

// ── Settings file ─────────────────────────────────────────────────────────────
pub const SETTINGS_JSON_PATH: &str = "settings.json";