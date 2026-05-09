use quartz::{tint_overlay, Arc, Font, Text, Span, Align, Color};
use crate::preferences::*;
use crate::rampstack::windows::*;
use flowmango::GameObject;
use quartz::Canvas;

pub fn ph_text(glyph: &str, size: f32, color: Color, font: Arc<Font>) -> Text {
    Text::new(
        vec![Span::new(
            glyph.to_string(),
            size,
            Some(size * 1.2),
            font,
            color,
            0.0,
        )],
        None,
        Align::Center,
        None,
    )
}

pub fn setup(cv: &mut Canvas, init_ch: f32, ph_bold: Arc<Font>) {
    let panel_top = TOPBAR_H + 1.0;
    let panel_h   = init_ch - panel_top;

    // ── Sidebar background + separator ────────────────────────────────────
    cv.add_game_object("sidebar_bg".into(), GameObject::build("sidebar_bg")
        .position(0.0, panel_top).size(SIDEBAR_W, panel_h).layer(4)
        .image(tint_overlay(SIDEBAR_W, panel_h, COL_SIDEBAR_BG)).finish());

    cv.add_game_object("sidebar_sep".into(), GameObject::build("sidebar_sep")
        .position(SIDEBAR_W, panel_top).size(1.0, panel_h).layer(4)
        .image(tint_overlay(1.0, panel_h, COL_BORDER)).finish());

    let icon_x      = (SIDEBAR_W - SIDEBAR_ICON_SIZE) * 0.5;
    let icon_col    = SIDEBAR_ICON_COLOR;
    let icon_active = SIDEBAR_ICON_COLOR_ACTIVE;
    let step        = SIDEBAR_ICON_SIZE + SIDEBAR_ICON_GAP;

    // Files — starts active since explorer is open on launch
    let mut file_obj = GameObject::build("sidebar_icon_files")
        .position(icon_x, panel_top + SIDEBAR_ICON_TOP)
        .size(SIDEBAR_ICON_SIZE, SIDEBAR_ICON_SIZE).layer(8).finish();
    file_obj.set_drawable(Box::new(ph_text(PH_FILES, SIDEBAR_ICON_SIZE, icon_active, ph_bold.clone())));
    cv.add_game_object("sidebar_icon_files".into(), file_obj);

    // Search
    let mut search_obj = GameObject::build("sidebar_icon_search")
        .position(icon_x, panel_top + SIDEBAR_ICON_TOP + step)
        .size(SIDEBAR_ICON_SIZE, SIDEBAR_ICON_SIZE).layer(8).finish();
    search_obj.set_drawable(Box::new(ph_text(PH_SEARCH, SIDEBAR_ICON_SIZE, icon_col, ph_bold.clone())));
    cv.add_game_object("sidebar_icon_search".into(), search_obj);

    // Users / collaborate
    let mut users_obj = GameObject::build("sidebar_icon_users")
        .position(icon_x, panel_top + SIDEBAR_ICON_TOP + step * 2.0)
        .size(SIDEBAR_ICON_SIZE, SIDEBAR_ICON_SIZE).layer(8).finish();
    users_obj.set_drawable(Box::new(ph_text(PH_USERS, SIDEBAR_ICON_SIZE, icon_col, ph_bold.clone())));
    cv.add_game_object("sidebar_icon_users".into(), users_obj);

    // Terminal
    let mut terminal_obj = GameObject::build("sidebar_icon_terminal")
        .position(icon_x, panel_top + SIDEBAR_ICON_TOP + step * 3.0)
        .size(SIDEBAR_ICON_SIZE, SIDEBAR_ICON_SIZE).layer(8).finish();
    terminal_obj.set_drawable(Box::new(ph_text(PH_TERMINAL, SIDEBAR_ICON_SIZE, icon_col, ph_bold.clone())));
    cv.add_game_object("sidebar_icon_terminal".into(), terminal_obj);

    // Git manager
    let mut git_obj = GameObject::build("sidebar_icon_git")
        .position(icon_x, panel_top + SIDEBAR_ICON_TOP + step * 4.0)
        .size(SIDEBAR_ICON_SIZE, SIDEBAR_ICON_SIZE).layer(8).finish();
    git_obj.set_drawable(Box::new(ph_text(PH_GIT, SIDEBAR_ICON_SIZE, icon_col, ph_bold.clone())));
    cv.add_game_object("sidebar_icon_git".into(), git_obj);

    // Extensions
    let mut ext_obj = GameObject::build("sidebar_icon_extensions")
        .position(icon_x, panel_top + SIDEBAR_ICON_TOP + step * 5.0)
        .size(SIDEBAR_ICON_SIZE, SIDEBAR_ICON_SIZE).layer(8).finish();
    ext_obj.set_drawable(Box::new(ph_text(PH_EXTENSIONS, SIDEBAR_ICON_SIZE, icon_col, ph_bold.clone())));
    cv.add_game_object("sidebar_icon_extensions".into(), ext_obj);

    // ── Canvas var ────────────────────────────────────────────────────────
    cv.set_var("sidebar_active", quartz::Value::from(SIDEBAR_FILES));
}

pub fn resize(cv: &mut Canvas, panel_h: f32) {
    if let Some(o) = cv.get_game_object_mut("sidebar_bg") {
        o.position = (0.0, TOPBAR_H + 1.0);
        if (o.size.1 - panel_h).abs() > 0.5 {
            o.size = (SIDEBAR_W, panel_h);
            o.set_image(tint_overlay(SIDEBAR_W, panel_h, COL_SIDEBAR_BG));
        }
    }
    if let Some(o) = cv.get_game_object_mut("sidebar_sep") {
        o.position = (SIDEBAR_W, TOPBAR_H + 1.0);
        if (o.size.1 - panel_h).abs() > 0.5 {
            o.size = (1.0, panel_h);
            o.set_image(tint_overlay(1.0, panel_h, COL_BORDER));
        }
    }
}