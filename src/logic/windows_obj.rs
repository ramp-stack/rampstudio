use quartz::{Arc, Font, Canvas};
use crate::preferences::*;
use crate::rampstack::windows::{
    divider_image_v, divider_image_v_color, divider_image_h,
    COL_DIVIDER_HOVER, DIV_W, HIT_HALF, MIN_EDITOR, MIN_TERMINAL, hit, Panels,
};
use crate::rampstack::windows::ICON_SIZE;
use crate::rampstack::windows::icon_rects;
use crate::objects::sidebar_obj::{ph_text, resize as sidebar_resize};

const REOPEN_RATIO: f32 = 0.22;

fn update_icon_images(cv: &mut Canvas, mode: u8) {
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
        let bytes = std::fs::read(stacked_path).unwrap_or_default();
        o.set_image(quartz::load_image_sized(&bytes, ICON_SIZE, ICON_SIZE));
    }
    if let Some(o) = cv.get_game_object_mut("icon_sidebyside") {
        let bytes = std::fs::read(sidebyside_path).unwrap_or_default();
        o.set_image(quartz::load_image_sized(&bytes, ICON_SIZE, ICON_SIZE));
    }
}

pub fn refresh_sidebar_icons(cv: &mut Canvas, active: u8, ph_bold: &Arc<Font>) {
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

fn set_divider_a_highlight(cv: &mut Canvas, highlighted: bool, panel_h: f32) {
    if let Some(o) = cv.get_game_object_mut("divider_a") {
        o.set_image(if highlighted {
            divider_image_v_color(panel_h, COL_DIVIDER_HOVER)
        } else {
            divider_image_v(panel_h)
        });
    }
}

fn set_divider_b_highlight(cv: &mut Canvas, highlighted: bool, panel_h: f32) {
    if let Some(o) = cv.get_game_object_mut("divider_b") {
        o.set_image(if highlighted {
            divider_image_v_color(panel_h, COL_DIVIDER_HOVER)
        } else {
            divider_image_v(panel_h)
        });
    }
}

fn set_divider_c_highlight(cv: &mut Canvas, highlighted: bool, right_w: f32) {
    if let Some(o) = cv.get_game_object_mut("divider_c") {
        use crate::rampstack::windows::divider_image_h;
        o.set_image(if highlighted {
            // horizontal divider hover — reuse make logic inline
            use quartz::{Image, ShapeType};
            use image::RgbaImage;
            let mut img = RgbaImage::new(1, DIV_W as u32);
            img.pixels_mut().for_each(|p| *p = image::Rgba(COL_DIVIDER_HOVER));
            Image {
                shape: ShapeType::Rectangle(0.0, (right_w, DIV_W), 0.0),
                image: img.into(),
                color: None,
            }
        } else {
            divider_image_h(right_w)
        });
    }
}

pub fn on_press(cv: &mut Canvas, mx: f32, my: f32, ph_bold: &Arc<Font>) -> bool {
    let panel_top = TOPBAR_H + 1.0;

    if mx <= SIDEBAR_W && my > panel_top {
        let icon_x  = (SIDEBAR_W - SIDEBAR_ICON_SIZE) * 0.5;
        let icon_y1 = panel_top + SIDEBAR_ICON_TOP;
        let icon_y2 = icon_y1 + SIDEBAR_ICON_SIZE + SIDEBAR_ICON_GAP;

        if hit(mx, my, icon_x, icon_y1, SIDEBAR_ICON_SIZE, SIDEBAR_ICON_SIZE) {
            let current = cv.get_u8("sidebar_active");
            let next = if current == SIDEBAR_FILES { SIDEBAR_NONE } else { SIDEBAR_FILES };
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
    let _ = ch;

    if my <= TOPBAR_H {
        let rects = icon_rects(cw);
        if hit(mx, my, rects[0].0, rects[0].1, ICON_SIZE, ICON_SIZE) {
            cv.set_var("layout_mode", quartz::Value::from(1u8));
            cv.set_var("drag_which",  quartz::Value::from(0u8));
            update_icon_images(cv, 1);
            return true;
        }
        if hit(mx, my, rects[1].0, rects[1].1, ICON_SIZE, ICON_SIZE) {
            cv.set_var("layout_mode", quartz::Value::from(0u8));
            cv.set_var("drag_which",  quartz::Value::from(0u8));
            update_icon_images(cv, 0);
            return true;
        }
        return false;
    }

    let mode           = cv.get_u8("layout_mode");
    let (cw, ch)       = cv.canvas_size();
    let panel_top      = TOPBAR_H + 1.0;
    let panel_h        = ch - panel_top;
    let a              = (cv.get_f32("ratio_a") * cw).round();
    let sidebar_active = cv.get_u8("sidebar_active");

    let div_a_x = if sidebar_active == SIDEBAR_FILES { a } else { SIDEBAR_W };
    if (mx - div_a_x).abs() <= HIT_HALF {
        cv.set_var("drag_which", quartz::Value::from(1u8));
        if sidebar_active != SIDEBAR_FILES {
            cv.set_var("sidebar_active", quartz::Value::from(SIDEBAR_FILES));
            refresh_sidebar_icons(cv, SIDEBAR_FILES, ph_bold);
            let min_explorer = cv.get_f32("min_explorer");
            let reopen_px    = (cw * REOPEN_RATIO).max(min_explorer);
            cv.set_var("ratio_a", quartz::Value::from(reopen_px / cw));
            cv.set_var("explorer_opening", quartz::Value::from(1u8));
        }
        set_divider_a_highlight(cv, true, panel_h);
        return true;
    }

    if mode == 0 {
        let b = (cv.get_f32("ratio_b") * cw).round();
        if (mx - b).abs() <= HIT_HALF {
            cv.set_var("drag_which", quartz::Value::from(2u8));
            set_divider_b_highlight(cv, true, panel_h);
            return true;
        }
    } else {
        let c_y = panel_top + (cv.get_f32("ratio_c") * panel_h).round();
        if (my - c_y).abs() <= HIT_HALF {
            cv.set_var("drag_which", quartz::Value::from(2u8));
            let right_w = cw - a - DIV_W;
            set_divider_c_highlight(cv, true, right_w);
            return true;
        }
    }

    cv.set_var("drag_which", quartz::Value::from(0u8));
    false
}

pub fn on_release(cv: &mut Canvas) {
    cv.set_var("drag_which",       quartz::Value::from(0u8));
    cv.set_var("explorer_opening", quartz::Value::from(0u8));
    let (cw, ch) = cv.canvas_size();
    let panel_h  = ch - TOPBAR_H - 1.0;
    let a        = (cv.get_f32("ratio_a") * cw).round();
    let right_w  = cw - a - DIV_W;
    // Clear all highlights; hover logic in on_move will restore if needed.
    set_divider_a_highlight(cv, false, panel_h);
    set_divider_b_highlight(cv, false, panel_h);
    set_divider_c_highlight(cv, false, right_w);
    cv.set_var("div_a_hover", quartz::Value::from(0u8));
    cv.set_var("div_b_hover", quartz::Value::from(0u8));
    cv.set_var("div_c_hover", quartz::Value::from(0u8));
}

pub fn on_move(cv: &mut Canvas, mx: f32, my: f32) {
    let which = cv.get_u8("drag_which");

    let (cw, ch)     = cv.canvas_size();
    let panel_top    = TOPBAR_H + 1.0;
    let panel_h      = ch - panel_top;
    let min_explorer = cv.get_f32("min_explorer");

    if which == 0 {
        let mode           = cv.get_u8("layout_mode");
        let sidebar_active = cv.get_u8("sidebar_active");
        let a              = (cv.get_f32("ratio_a") * cw).round();
        let right_w        = cw - a - DIV_W;

        // ── Divider A hover ───────────────────────────────────────────────
        let div_a_x    = if sidebar_active == SIDEBAR_FILES { a } else { SIDEBAR_W };
        let hover_a    = (mx - div_a_x).abs() <= HIT_HALF && my > panel_top;
        let was_a      = cv.has_var("div_a_hover") && cv.get_u8("div_a_hover") != 0;
        if hover_a != was_a {
            cv.set_var("div_a_hover", quartz::Value::from(hover_a as u8));
            set_divider_a_highlight(cv, hover_a, panel_h);
        }

        // ── Divider B hover (side-by-side only) ───────────────────────────
        if mode == 0 {
            let b      = (cv.get_f32("ratio_b") * cw).round();
            let hover_b = (mx - b).abs() <= HIT_HALF && my > panel_top;
            let was_b   = cv.has_var("div_b_hover") && cv.get_u8("div_b_hover") != 0;
            if hover_b != was_b {
                cv.set_var("div_b_hover", quartz::Value::from(hover_b as u8));
                set_divider_b_highlight(cv, hover_b, panel_h);
            }
        }

        // ── Divider C hover (stacked only) ────────────────────────────────
        if mode == 1 {
            let c_y    = panel_top + (cv.get_f32("ratio_c") * panel_h).round();
            let hover_c = (my - c_y).abs() <= HIT_HALF && mx > SIDEBAR_W;
            let was_c   = cv.has_var("div_c_hover") && cv.get_u8("div_c_hover") != 0;
            if hover_c != was_c {
                cv.set_var("div_c_hover", quartz::Value::from(hover_c as u8));
                set_divider_c_highlight(cv, hover_c, right_w);
            }
        }

        return;
    }

    let mode = cv.get_u8("layout_mode");

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

pub fn update(cv: &mut Canvas, ph_bold: &Arc<Font>) -> Panels {
    let (cw, ch)       = cv.canvas_size();
    let panel_top      = TOPBAR_H + 1.0;
    let panel_h        = ch - panel_top;
    let mode           = cv.get_u8("layout_mode");
    let min_explorer   = cv.get_f32("min_explorer");
    let sidebar_active = cv.get_u8("sidebar_active");
    let drag_which     = cv.get_u8("drag_which");
    let explorer_visible = sidebar_active == SIDEBAR_FILES;

    sidebar_resize(cv, panel_h);

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

    let opening = cv.has_var("explorer_opening") && cv.get_u8("explorer_opening") != 0;
    let explorer_visible = if explorer_visible
        && drag_which == 1
        && !opening
        && a <= (min_ra * cw).round() + 40.0
    {
        cv.set_var("sidebar_active", quartz::Value::from(SIDEBAR_NONE));
        refresh_sidebar_icons(cv, SIDEBAR_NONE, ph_bold);
        false
    } else {
        if opening && a > (min_ra * cw).round() + 60.0 {
            cv.set_var("explorer_opening", quartz::Value::from(0u8));
        }
        explorer_visible
    };

    let explorer_right = if explorer_visible { a } else { SIDEBAR_W };
    let right_x = explorer_right + DIV_W;
    let right_w = cw - right_x;

    // ── Background ────────────────────────────────────────────────────────
    if let Some(o) = cv.get_game_object_mut("app_bg") {
        if o.size != (cw, ch) {
            o.size = (cw, ch);
            o.set_image(quartz::tint_overlay(cw, ch, COL_APP_BG));
        }
    }

    // ── Topbar ────────────────────────────────────────────────────────────
    if let Some(o) = cv.get_game_object_mut("topbar_bg") {
        if (o.size.0 - cw).abs() > 0.5 {
            o.size = (cw, TOPBAR_H);
            o.set_image(quartz::tint_overlay(cw, TOPBAR_H, COL_TOPBAR_BG));
        }
    }
    if let Some(o) = cv.get_game_object_mut("topbar_sep") {
        if (o.size.0 - cw).abs() > 0.5 {
            o.size = (cw, 1.0);
            o.set_image(quartz::tint_overlay(cw, 1.0, COL_BORDER));
        }
        o.position = (0.0, TOPBAR_H);
    }

    // ── Layout icons reposition ───────────────────────────────────────────
    let rects = icon_rects(cw);
    if let Some(o) = cv.get_game_object_mut("icon_stacked")    { o.position = (rects[0].0, rects[0].1); }
    if let Some(o) = cv.get_game_object_mut("icon_sidebyside") { o.position = (rects[1].0, rects[1].1); }

    // ── Divider A ─────────────────────────────────────────────────────────
    let div_a_x        = if explorer_visible { a } else { SIDEBAR_W };
    let div_a_hovering = cv.has_var("div_a_hover") && cv.get_u8("div_a_hover") != 0;
    if let Some(o) = cv.get_game_object_mut("divider_a") {
        o.visible  = true;
        o.position = (div_a_x, panel_top);
        if (o.size.1 - panel_h).abs() > 0.5 {
            o.size = (DIV_W, panel_h);
            o.set_image(if div_a_hovering {
                divider_image_v_color(panel_h, COL_DIVIDER_HOVER)
            } else {
                divider_image_v(panel_h)
            });
        }
    }

    // ── Mode-specific ─────────────────────────────────────────────────────
    if mode == 0 {
        let b             = (rb * cw).round();
        let div_b_hovered = cv.has_var("div_b_hover") && cv.get_u8("div_b_hover") != 0;

        if let Some(o) = cv.get_game_object_mut("divider_b") {
            o.visible  = true;
            o.position = (b, panel_top);
            if (o.size.1 - panel_h).abs() > 0.5 {
                o.size = (DIV_W, panel_h);
                o.set_image(if div_b_hovered {
                    divider_image_v_color(panel_h, COL_DIVIDER_HOVER)
                } else {
                    divider_image_v(panel_h)
                });
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
        let c_y           = panel_top + (rc * panel_h).round();
        let div_c_hovered = cv.has_var("div_c_hover") && cv.get_u8("div_c_hover") != 0;

        if let Some(o) = cv.get_game_object_mut("divider_b") { o.visible = false; }
        if let Some(o) = cv.get_game_object_mut("divider_c") {
            o.visible  = true;
            o.position = (right_x, c_y);
            if (o.size.0 - right_w).abs() > 0.5 {
                o.size = (right_w, DIV_W);
                o.set_image(if div_c_hovered {
                    // horizontal hover image
                    use quartz::{Image, ShapeType};
                    use image::RgbaImage;
                    let mut img = RgbaImage::new(1, DIV_W as u32);
                    img.pixels_mut().for_each(|p| *p = image::Rgba(COL_DIVIDER_HOVER));
                    Image {
                        shape: ShapeType::Rectangle(0.0, (right_w, DIV_W), 0.0),
                        image: img.into(),
                        color: None,
                    }
                } else {
                    divider_image_h(right_w)
                });
            }
        }

        let tm_y = c_y + DIV_W;
        Panels {
            explorer: (SIDEBAR_W, panel_top, a - SIDEBAR_W, panel_h),
            editor:   (right_x,  panel_top,  right_w,       c_y - panel_top),
            terminal: (right_x,  tm_y,       right_w,       ch - tm_y),
            explorer_visible,
        }
    }
}