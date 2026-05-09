use flowmango::GameObject;
use quartz::{load_image_sized, tint_overlay};
use crate::settings;
use crate::preferences::{
    SIDEBAR_W, SIDEBAR_FILES, SIDEBAR_NONE,
    SIDEBAR_ICON_SIZE, SIDEBAR_ICON_COLOR, SIDEBAR_ICON_COLOR_ACTIVE,
    SIDEBAR_ICON_TOP, SIDEBAR_ICON_GAP,
    PH_FILES, PH_SEARCH,
};
use crate::objects::sidebar_obj::{ph_text, resize as sidebar_resize};
use quartz::ShapeType;
use quartz::Image;
use quartz::Font;
use std::sync::Arc;

pub const DIV_W:    f32 = 2.0;
pub const HIT_HALF: f32 = 5.0;

pub const MIN_EXPLORER: f32 = 120.0;
pub const MIN_EDITOR:   f32 = 200.0;
pub const MIN_TERMINAL: f32 = 160.0;

// How wide the explorer snaps to when reopened via the sidebar icon.
const REOPEN_RATIO: f32 = 0.22;

// Icon sizing
pub const ICON_SIZE: f32 = 30.0;
const ICON_PAD:  f32 = 10.0;
const ICON_GAP:  f32 = 10.0;

// Logo
const LOGO_H:   f32 = 80.0;
const LOGO_W:   f32 = 80.0;
const LOGO_PAD: f32 = 12.0;

// Divider colours
const COL_DIVIDER_NORMAL: [u8; 4] = [40, 40, 40, 255];
pub const COL_DIVIDER_HOVER: [u8; 4] = [255, 255, 255, 180];

// ── Image helpers ──────────────────────────────────────────────────────────

fn make_divider_v(h: f32, rgba: [u8; 4]) -> Image {
    use image::RgbaImage;
    let mut img = RgbaImage::new(DIV_W as u32, 1);
    img.pixels_mut().for_each(|p| *p = image::Rgba(rgba));
    Image {
        shape: ShapeType::Rectangle(0.0, (DIV_W, h), 0.0),
        image: img.into(),
        color: None,
    }
}

pub fn divider_image_v(h: f32) -> Image {
    make_divider_v(h, COL_DIVIDER_NORMAL)
}

pub fn divider_image_v_hover(h: f32) -> Image {
    make_divider_v(h, COL_DIVIDER_HOVER)
}

pub fn divider_image_v_color(h: f32, rgba: [u8; 4]) -> Image {
    make_divider_v(h, rgba)
}

pub fn divider_image_h(w: f32) -> Image {
    use image::RgbaImage;
    let mut img = RgbaImage::new(1, DIV_W as u32);
    img.pixels_mut().for_each(|p| *p = image::Rgba(COL_DIVIDER_NORMAL));
    Image {
        shape: ShapeType::Rectangle(0.0, (w, DIV_W), 0.0),
        image: img.into(),
        color: None,
    }
}

// ── Layout helpers ─────────────────────────────────────────────────────────

pub fn icon_rects(cw: f32) -> [(f32, f32); 2] {
    let y  = (settings::TOPBAR_H - ICON_SIZE) * 0.5;
    let x1 = cw - ICON_PAD - ICON_SIZE;
    let x0 = x1 - ICON_GAP - ICON_SIZE;
    [(x0, y), (x1, y)]
}

pub fn hit(mx: f32, my: f32, x: f32, y: f32, w: f32, h: f32) -> bool {
    mx >= x && mx <= x + w && my >= y && my <= y + h
}

fn update_layout_icon_images(cv: &mut quartz::Canvas, mode: u8) {
    let stacked_path = if mode == 1 {
        "resources/selected_stacked.png"
    } else {
        "resources/unselected_stacked.png"
    };
    let sidebyside_path = if mode == 0 {
        "resources/selected_sidebyside.png"
    } else {
        "resources/unselected_sidebyside.png"
    };
    if let Some(o) = cv.get_game_object_mut("icon_stacked") {
        o.set_image(load_image_sized(stacked_path, ICON_SIZE, ICON_SIZE));
    }
    if let Some(o) = cv.get_game_object_mut("icon_sidebyside") {
        o.set_image(load_image_sized(sidebyside_path, ICON_SIZE, ICON_SIZE));
    }
}

pub fn refresh_sidebar_icons(cv: &mut quartz::Canvas, active: u8, ph_bold: &Arc<Font>) {
    let files_col = if active == SIDEBAR_FILES {
        SIDEBAR_ICON_COLOR_ACTIVE
    } else {
        SIDEBAR_ICON_COLOR
    };
    if let Some(o) = cv.get_game_object_mut("sidebar_icon_files") {
        o.set_drawable(Box::new(ph_text(PH_FILES, SIDEBAR_ICON_SIZE, files_col, ph_bold.clone())));
    }
    if let Some(o) = cv.get_game_object_mut("sidebar_icon_search") {
        o.set_drawable(Box::new(ph_text(PH_SEARCH, SIDEBAR_ICON_SIZE, SIDEBAR_ICON_COLOR, ph_bold.clone())));
    }
}

fn apply_divider_a_color(cv: &mut quartz::Canvas, highlighted: bool, panel_h: f32) {
    if let Some(o) = cv.get_game_object_mut("divider_a") {
        o.set_image(if highlighted {
            divider_image_v_hover(panel_h)
        } else {
            divider_image_v(panel_h)
        });
    }
}

// ── mount ──────────────────────────────────────────────────────────────────

pub fn mount(cv: &mut quartz::Canvas, init_cw: f32, init_ch: f32, ratio_a: f32, ratio_b: f32) {
    let split_a   = (init_cw * ratio_a).round();
    let split_b   = (init_cw * ratio_b).round();
    let panel_top = settings::TOPBAR_H + 1.0;
    let panel_h   = init_ch - panel_top;

    // Background
    cv.add_game_object("app_bg".into(), GameObject::build("app_bg")
        .position(0.0, 0.0).size(4000.0, 4000.0).layer(-1)
        .image(tint_overlay(4000.0, 4000.0, settings::COL_APP_BG)).finish());

    // Topbar
    cv.add_game_object("topbar_bg".into(), GameObject::build("topbar_bg")
        .position(0.0, 0.0).size(init_cw, settings::TOPBAR_H).layer(5)
        .image(tint_overlay(init_cw, settings::TOPBAR_H, settings::COL_TOPBAR_BG)).finish());

    cv.add_game_object("topbar_sep".into(), GameObject::build("topbar_sep")
        .position(0.0, settings::TOPBAR_H).size(init_cw, 1.0).layer(6)
        .image(tint_overlay(init_cw, 1.0, settings::COL_BORDER)).finish());

    // Logo
    let logo_img = load_image_sized("resources/rampstacklogo.png", LOGO_W, LOGO_H);
    let logo_y   = (settings::TOPBAR_H - LOGO_H) * 0.5;
    cv.add_game_object("topbar_logo".into(), GameObject::build("topbar_logo")
        .position(LOGO_PAD, logo_y).size(LOGO_W, LOGO_H).layer(7)
        .image(logo_img).finish());

    // Layout-mode icons
    let rects          = icon_rects(init_cw);
    let stacked_img    = load_image_sized("resources/unselected_stacked.png",  ICON_SIZE, ICON_SIZE);
    let sidebyside_img = load_image_sized("resources/selected_sidebyside.png", ICON_SIZE, ICON_SIZE);

    cv.add_game_object("icon_stacked".into(), GameObject::build("icon_stacked")
        .position(rects[0].0, rects[0].1).size(ICON_SIZE, ICON_SIZE).layer(7)
        .image(stacked_img).finish());

    cv.add_game_object("icon_sidebyside".into(), GameObject::build("icon_sidebyside")
        .position(rects[1].0, rects[1].1).size(ICON_SIZE, ICON_SIZE).layer(7)
        .image(sidebyside_img).finish());

    // Divider A
    let mut div_a = GameObject::build("divider_a")
        .position(split_a, panel_top).size(DIV_W, panel_h).layer(10).finish();
    div_a.set_image(divider_image_v(panel_h));
    cv.add_game_object("divider_a".into(), div_a);

    // Divider B
    let mut div_b = GameObject::build("divider_b")
        .position(split_b, panel_top).size(DIV_W, panel_h).layer(10).finish();
    div_b.set_image(divider_image_v(panel_h));
    cv.add_game_object("divider_b".into(), div_b);

    // Divider C (stacked mode, starts hidden)
    let right_w      = init_cw - split_a - DIV_W;
    let init_ratio_c = 0.6f32;
    let split_c_y    = panel_top + (panel_h * init_ratio_c).round();
    let mut div_c = GameObject::build("divider_c")
        .position(split_a + DIV_W, split_c_y).size(right_w, DIV_W).layer(10).finish();
    div_c.set_image(divider_image_h(right_w));
    div_c.visible = false;
    cv.add_game_object("divider_c".into(), div_c);

    // Canvas vars
    cv.set_var("ratio_a",          quartz::Value::from(ratio_a));
    cv.set_var("ratio_b",          quartz::Value::from(ratio_b));
    cv.set_var("ratio_c",          quartz::Value::from(init_ratio_c));
    cv.set_var("drag_which",       quartz::Value::from(0u8));
    cv.set_var("layout_mode",      quartz::Value::from(0u8));
    cv.set_var("min_explorer",     quartz::Value::from(MIN_EXPLORER));
    cv.set_var("div_a_hover",      quartz::Value::from(0u8));
    cv.set_var("explorer_opening", quartz::Value::from(0u8));
}

// ── on_press ───────────────────────────────────────────────────────────────

pub fn on_press(cv: &mut quartz::Canvas, mx: f32, my: f32, ph_bold: &Arc<Font>) -> bool {
    let panel_top = settings::TOPBAR_H + 1.0;

    // ── Sidebar icon hit tests ────────────────────────────────────────────
    if mx <= SIDEBAR_W && my > panel_top {
        let icon_x  = (SIDEBAR_W - SIDEBAR_ICON_SIZE) * 0.5;
        let icon_y1 = panel_top + SIDEBAR_ICON_TOP;
        let icon_y2 = icon_y1 + SIDEBAR_ICON_SIZE + SIDEBAR_ICON_GAP;

        if hit(mx, my, icon_x, icon_y1, SIDEBAR_ICON_SIZE, SIDEBAR_ICON_SIZE) {
            let current = cv.get_u8("sidebar_active");
            let next    = if current == SIDEBAR_FILES { SIDEBAR_NONE } else { SIDEBAR_FILES };
            cv.set_var("sidebar_active", quartz::Value::from(next));
            refresh_sidebar_icons(cv, next, ph_bold);
            if next == SIDEBAR_FILES {
                let (cw, _)      = cv.canvas_size();
                let min_explorer = cv.get_f32("min_explorer");
                let reopen_px    = (cw * REOPEN_RATIO).max(min_explorer);
                cv.set_var("ratio_a", quartz::Value::from(reopen_px / cw));
            }
            return true;
        }
        if hit(mx, my, icon_x, icon_y2, SIDEBAR_ICON_SIZE, SIDEBAR_ICON_SIZE) {
            return true;
        }
        return true;
    }

    let (cw, ch) = cv.canvas_size();

    // ── Topbar layout icons ───────────────────────────────────────────────
    if my <= settings::TOPBAR_H {
        let rects = icon_rects(cw);
        if hit(mx, my, rects[0].0, rects[0].1, ICON_SIZE, ICON_SIZE) {
            cv.set_var("layout_mode", quartz::Value::from(1u8));
            cv.set_var("drag_which",  quartz::Value::from(0u8));
            update_layout_icon_images(cv, 1);
            return true;
        }
        if hit(mx, my, rects[1].0, rects[1].1, ICON_SIZE, ICON_SIZE) {
            cv.set_var("layout_mode", quartz::Value::from(0u8));
            cv.set_var("drag_which",  quartz::Value::from(0u8));
            update_layout_icon_images(cv, 0);
            return true;
        }
        return false;
    }

    // ── Divider hit tests ─────────────────────────────────────────────────
    let mode           = cv.get_u8("layout_mode");
    let panel_h        = ch - panel_top;
    let sidebar_active = cv.get_u8("sidebar_active");
    let a              = (cv.get_f32("ratio_a") * cw).round();

    let div_a_x = if sidebar_active == SIDEBAR_FILES { a } else { SIDEBAR_W };
    if (mx - div_a_x).abs() <= HIT_HALF {
        cv.set_var("drag_which", quartz::Value::from(1u8));
        if sidebar_active != SIDEBAR_FILES {
            cv.set_var("sidebar_active", quartz::Value::from(SIDEBAR_FILES));
            refresh_sidebar_icons(cv, SIDEBAR_FILES, ph_bold);
            let min_explorer = cv.get_f32("min_explorer");
            let reopen_px    = (cw * REOPEN_RATIO).max(min_explorer);
            cv.set_var("ratio_a", quartz::Value::from(reopen_px / cw));
            // Suppress auto-close for this drag
            cv.set_var("explorer_opening", quartz::Value::from(1u8));
        }
        apply_divider_a_color(cv, true, panel_h);
        return true;
    }

    if mode == 0 {
        let b = (cv.get_f32("ratio_b") * cw).round();
        if (mx - b).abs() <= HIT_HALF {
            cv.set_var("drag_which", quartz::Value::from(2u8));
            return true;
        }
    } else {
        let c_y = panel_top + (cv.get_f32("ratio_c") * panel_h).round();
        if (my - c_y).abs() <= HIT_HALF {
            cv.set_var("drag_which", quartz::Value::from(2u8));
            return true;
        }
    }

    cv.set_var("drag_which", quartz::Value::from(0u8));
    false
}

// ── on_release ─────────────────────────────────────────────────────────────

pub fn on_release(cv: &mut quartz::Canvas) {
    cv.set_var("drag_which",       quartz::Value::from(0u8));
    cv.set_var("explorer_opening", quartz::Value::from(0u8));
    let (_, ch) = cv.canvas_size();
    let panel_h  = ch - settings::TOPBAR_H - 1.0;
    cv.set_var("div_a_hover", quartz::Value::from(0u8));
    apply_divider_a_color(cv, false, panel_h);
}

// ── on_move ────────────────────────────────────────────────────────────────

pub fn on_move(cv: &mut quartz::Canvas, mx: f32, my: f32) {
    let which = cv.get_u8("drag_which");

    let (cw, ch)  = cv.canvas_size();
    let panel_top = settings::TOPBAR_H + 1.0;
    let panel_h   = ch - panel_top;

    // ── Hover highlight for divider A (idle state only) ───────────────────
    if which == 0 {
        let sidebar_active = cv.get_u8("sidebar_active");
        let a              = (cv.get_f32("ratio_a") * cw).round();
        let div_a_x        = if sidebar_active == SIDEBAR_FILES { a } else { SIDEBAR_W };
        let hovering       = (mx - div_a_x).abs() <= HIT_HALF && my > panel_top;
        let was_hovering   = cv.get_u8("div_a_hover") != 0;

        if hovering != was_hovering {
            cv.set_var("div_a_hover", quartz::Value::from(hovering as u8));
            apply_divider_a_color(cv, hovering, panel_h);
        }
        return;
    }

    // ── Drag ──────────────────────────────────────────────────────────────
    let mode         = cv.get_u8("layout_mode");
    let min_explorer = cv.get_f32("min_explorer");

    match which {
        1 => {
            let min_a = min_explorer;
            let max_a = if mode == 0 {
                let b_px = (cv.get_f32("ratio_b") * cw).round();
                (b_px - DIV_W - MIN_EDITOR).max(min_a)
            } else {
                (cw - DIV_W - MIN_EDITOR).max(min_a)
            };
            cv.set_var("ratio_a", quartz::Value::from(mx.clamp(min_a, max_a) / cw));
        }
        2 if mode == 0 => {
            let a_px  = (cv.get_f32("ratio_a") * cw).round();
            let min_b = a_px + DIV_W + MIN_EDITOR;
            let max_b = (cw - DIV_W - MIN_TERMINAL).max(min_b);
            cv.set_var("ratio_b", quartz::Value::from(mx.clamp(min_b, max_b) / cw));
        }
        2 => {
            let min_y = panel_top + MIN_EDITOR;
            let max_y = (panel_top + panel_h - DIV_W - MIN_TERMINAL).max(min_y);
            let py    = my.clamp(min_y, max_y);
            cv.set_var("ratio_c", quartz::Value::from((py - panel_top) / panel_h));
        }
        _ => {}
    }
}

// ── Panels ─────────────────────────────────────────────────────────────────

pub struct Panels {
    pub explorer:         (f32, f32, f32, f32),
    pub editor:           (f32, f32, f32, f32),
    pub terminal:         (f32, f32, f32, f32),
    pub explorer_visible: bool,
}

// ── update ─────────────────────────────────────────────────────────────────

pub fn update(cv: &mut quartz::Canvas, ph_bold: &Arc<Font>) -> Panels {
    let (cw, ch)       = cv.canvas_size();
    let panel_top      = settings::TOPBAR_H + 1.0;
    let panel_h        = ch - panel_top;
    let mode           = cv.get_u8("layout_mode");
    let min_explorer   = cv.get_f32("min_explorer");
    let sidebar_active = cv.get_u8("sidebar_active");
    let drag_which     = cv.get_u8("drag_which");
    let explorer_visible = sidebar_active == SIDEBAR_FILES;

    // ── Sidebar resize ────────────────────────────────────────────────────
    sidebar_resize(cv, panel_h);

    // ── Clamp ratio_b first so ratio_a's max is derived from a valid b ───
    let rb = if mode == 0 {
        let raw     = cv.get_f32("ratio_b");
        let min_rb  = (min_explorer + DIV_W + MIN_EDITOR) / cw;
        let max_rb  = 1.0 - (DIV_W + MIN_TERMINAL) / cw;
        let clamped = raw.clamp(min_rb, max_rb);
        cv.set_var("ratio_b", quartz::Value::from(clamped));
        clamped
    } else {
        cv.get_f32("ratio_b")
    };

    // ── Clamp ratio_a ─────────────────────────────────────────────────────
    let min_ra = min_explorer / cw;
    let max_ra = if mode == 0 {
        let b_px = (rb * cw).round();
        ((b_px - DIV_W - MIN_EDITOR) / cw).max(min_ra)
    } else {
        ((cw - DIV_W - MIN_EDITOR) / cw).max(min_ra)
    };
    let ra = cv.get_f32("ratio_a").clamp(min_ra, max_ra);
    cv.set_var("ratio_a", quartz::Value::from(ra));
    let a = (ra * cw).round();

    // ── Auto-close when dragged to minimum ────────────────────────────────
    let opening = cv.get_u8("explorer_opening") != 0;
    let explorer_visible = if explorer_visible
        && drag_which == 1
        && !opening
        && a <= (min_ra * cw).round() + 40.0
    {
        cv.set_var("sidebar_active", quartz::Value::from(SIDEBAR_NONE));
        refresh_sidebar_icons(cv, SIDEBAR_NONE, ph_bold);
        false
    } else {
        // Clear the opening flag once dragged comfortably past the threshold
        if opening && a > (min_ra * cw).round() + 60.0 {
            cv.set_var("explorer_opening", quartz::Value::from(0u8));
        }
        explorer_visible
    };

    let explorer_right = if explorer_visible { a } else { SIDEBAR_W };
    let right_x        = explorer_right + DIV_W;
    let right_w        = cw - right_x;

    // ── Background ────────────────────────────────────────────────────────
    if let Some(o) = cv.get_game_object_mut("app_bg") {
        if o.size != (cw, ch) {
            o.size = (cw, ch);
            o.set_image(tint_overlay(cw, ch, settings::COL_APP_BG));
        }
    }

    // ── Topbar ────────────────────────────────────────────────────────────
    if let Some(o) = cv.get_game_object_mut("topbar_bg") {
        if (o.size.0 - cw).abs() > 0.5 {
            o.size = (cw, settings::TOPBAR_H);
            o.set_image(tint_overlay(cw, settings::TOPBAR_H, settings::COL_TOPBAR_BG));
        }
    }
    if let Some(o) = cv.get_game_object_mut("topbar_sep") {
        if (o.size.0 - cw).abs() > 0.5 {
            o.size = (cw, 1.0);
            o.set_image(tint_overlay(cw, 1.0, settings::COL_BORDER));
        }
        o.position = (0.0, settings::TOPBAR_H);
    }

    // ── Layout icons — reposition only ───────────────────────────────────
    let rects = icon_rects(cw);
    if let Some(o) = cv.get_game_object_mut("icon_stacked")    { o.position = (rects[0].0, rects[0].1); }
    if let Some(o) = cv.get_game_object_mut("icon_sidebyside") { o.position = (rects[1].0, rects[1].1); }

    // ── Divider A — always visible; parked at sidebar edge when closed ────
    let div_a_x      = if explorer_visible { a } else { SIDEBAR_W };
    let div_a_hovered = cv.get_u8("div_a_hover") != 0;
    if let Some(o) = cv.get_game_object_mut("divider_a") {
        o.visible  = true;
        o.position = (div_a_x, panel_top);
        if (o.size.1 - panel_h).abs() > 0.5 {
            o.size = (DIV_W, panel_h);
            o.set_image(if div_a_hovered {
                divider_image_v_hover(panel_h)
            } else {
                divider_image_v(panel_h)
            });
        }
    }

    // ── Mode-specific ─────────────────────────────────────────────────────
    if mode == 0 {
        let b = (rb * cw).round();

        if let Some(o) = cv.get_game_object_mut("divider_b") {
            o.visible  = true;
            o.position = (b, panel_top);
            if (o.size.1 - panel_h).abs() > 0.5 {
                o.size = (DIV_W, panel_h);
                o.set_image(divider_image_v(panel_h));
            }
        }
        if let Some(o) = cv.get_game_object_mut("divider_c") { o.visible = false; }

        let (editor_x, editor_w, terminal_x, terminal_w) = if explorer_visible {
            (right_x, b - right_x, b + DIV_W, cw - b - DIV_W)
        } else {
            let b2 = (rb * cw).round();
            (SIDEBAR_W + DIV_W, b2 - SIDEBAR_W - DIV_W, b2 + DIV_W, cw - b2 - DIV_W)
        };

        Panels {
            explorer: (SIDEBAR_W,  panel_top, a - SIDEBAR_W, panel_h),
            editor:   (editor_x,   panel_top, editor_w,      panel_h),
            terminal: (terminal_x, panel_top, terminal_w,    panel_h),
            explorer_visible,
        }
    } else {
        let rc = {
            let raw    = cv.get_f32("ratio_c");
            let min_rc = MIN_EDITOR / panel_h;
            let max_rc = 1.0 - (DIV_W + MIN_TERMINAL) / panel_h;
            let c      = raw.clamp(min_rc, max_rc);
            cv.set_var("ratio_c", quartz::Value::from(c));
            c
        };
        let c_y = panel_top + (rc * panel_h).round();

        if let Some(o) = cv.get_game_object_mut("divider_b") { o.visible = false; }
        if let Some(o) = cv.get_game_object_mut("divider_c") {
            o.visible  = true;
            o.position = (right_x, c_y);
            if (o.size.0 - right_w).abs() > 0.5 {
                o.size = (right_w, DIV_W);
                o.set_image(divider_image_h(right_w));
            }
        }

        let tm_y = c_y + DIV_W;
        Panels {
            explorer: (SIDEBAR_W, panel_top, a - SIDEBAR_W, panel_h),
            editor:   (right_x,   panel_top, right_w,       c_y - panel_top),
            terminal: (right_x,   tm_y,      right_w,       ch - tm_y),
            explorer_visible,
        }
    }
}