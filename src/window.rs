use quartz::{Arc, Canvas, Font};
use window_layout::{
    SetupConfig, SidebarIcon, StatusBarItem, StatusBarAlign,
    setup::mount,
    logic::{on_press, on_release, on_move, update},
};
pub use window_layout::layout::Panels;

use crate::preferences::*;
use crate::theme::ChromeTheme;

const PH_SB_USER: &str = "\u{e61a}";
const PH_SB_FILE: &str = "\u{e295}";

pub fn build_config(
    icon_font: Arc<Font>,
    text_font: Arc<Font>,
    chrome:    &ChromeTheme,
) -> SetupConfig {
    let mut cfg = SetupConfig::new(icon_font);

    cfg.statusbar_font = text_font;
    cfg.topbar_h    = TOPBAR_H;
    cfg.sidebar_w   = SIDEBAR_W;
    cfg.statusbar_h = 22.0;

    cfg.col_app_bg     = chrome.app_bg;
    cfg.col_topbar_bg  = chrome.topbar_bg;
    cfg.col_sidebar_bg = chrome.sidebar_bg;
    cfg.col_border     = chrome.border;

    cfg.col_statusbar_bg    = chrome.statusbar_bg;
    cfg.col_statusbar_fg    = chrome.statusbar_fg;
    cfg.col_statusbar_hover = chrome.statusbar_hover;

    cfg.logo_bytes                  = include_bytes!("../resources/rampstacklogo.png");
    cfg.icon_stacked_bytes          = include_bytes!("../resources/unselected_stacked.png");
    cfg.icon_stacked_selected_bytes = include_bytes!("../resources/selected_stacked.png");
    cfg.icon_sidebyside_bytes       = include_bytes!("../resources/selected_sidebyside.png");
    cfg.icon_sidebyside_unsel_bytes = include_bytes!("../resources/unselected_sidebyside.png");

    cfg.init_ratio_a = INIT_EXPLORER_RATIO;
    cfg.init_ratio_b = 1.0 - INIT_TERMINAL_RATIO;
    cfg.init_ratio_c = 0.60;
    cfg.reopen_ratio = 0.22;

    cfg.sidebar_icon_size         = SIDEBAR_ICON_SIZE;
    cfg.sidebar_icon_top          = SIDEBAR_ICON_TOP;
    cfg.sidebar_icon_gap          = SIDEBAR_ICON_GAP;
    cfg.sidebar_icon_color        = chrome.sidebar_icon;
    cfg.sidebar_icon_color_active = chrome.sidebar_icon_active;

    // ALL icons have opens_panel: true so clicking any icon opens the
    // left panel slot. The app then swaps what's rendered inside it.
    cfg.sidebar_icons = vec![
        SidebarIcon { key: "sidebar_icon_files",      glyph: PH_FILES,      sidebar_id: ICON_ID_FILES,      opens_panel: true },
        SidebarIcon { key: "sidebar_icon_search",     glyph: PH_SEARCH,     sidebar_id: ICON_ID_SEARCH,     opens_panel: true },
        SidebarIcon { key: "sidebar_icon_users",      glyph: PH_USERS,      sidebar_id: ICON_ID_USERS,      opens_panel: true },
        SidebarIcon { key: "sidebar_icon_terminal",   glyph: PH_TERMINAL,   sidebar_id: ICON_ID_TERMINAL,   opens_panel: true },
        SidebarIcon { key: "sidebar_icon_git",        glyph: PH_GIT,        sidebar_id: ICON_ID_GIT,        opens_panel: true },
        SidebarIcon { key: "sidebar_icon_extensions", glyph: PH_EXTENSIONS, sidebar_id: ICON_ID_EXTENSIONS, opens_panel: true },
    ];

    cfg.statusbar_items = vec![
        StatusBarItem { key: "sb_branch",   glyph: PH_GIT,     initial: "",            align: StatusBarAlign::Left,  clickable: false },
        StatusBarItem { key: "sb_commit",   glyph: PH_SB_USER, initial: "",            align: StatusBarAlign::Left,  clickable: false },
        StatusBarItem { key: "sb_cursor",   glyph: "",         initial: "Ln 1, Col 1", align: StatusBarAlign::Right, clickable: false },
        StatusBarItem { key: "sb_lang",     glyph: PH_SB_FILE, initial: "",            align: StatusBarAlign::Right, clickable: false },
        StatusBarItem { key: "sb_encoding", glyph: "",         initial: "UTF-8",       align: StatusBarAlign::Right, clickable: false },
    ];

    cfg.statusbar_font_size = 11.0;
    cfg.statusbar_item_pad  = 14.0;
    cfg.statusbar_item_gap  = 10.0;
    cfg.initial_active      = ICON_ID_FILES;
    cfg
}

pub fn setup(cv: &mut Canvas, cw: f32, ch: f32, cfg: &SetupConfig) { mount(cv, cw, ch, cfg); }
pub fn handle_press(cv: &mut Canvas, mx: f32, my: f32, cfg: &SetupConfig) -> bool { on_press(cv, mx, my, cfg) }
pub fn handle_release(cv: &mut Canvas, cfg: &SetupConfig) { on_release(cv, cfg); }
pub fn handle_move(cv: &mut Canvas, mx: f32, my: f32) { on_move(cv, mx, my); }
pub fn tick(cv: &mut Canvas, cfg: &SetupConfig) -> Panels { update(cv, cfg) }