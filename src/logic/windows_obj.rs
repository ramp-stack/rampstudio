use quartz::{Canvas, load_image_sized};
use crate::preferences::*;
use crate::rampstack::windows::*;

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
        o.set_image(load_image_sized(stacked_path, ICON_SIZE, ICON_SIZE));
    }
    if let Some(o) = cv.get_game_object_mut("icon_sidebyside") {
        o.set_image(load_image_sized(sidebyside_path, ICON_SIZE, ICON_SIZE));
    }
}

pub fn on_press(cv: &mut Canvas, mx: f32, my: f32) -> bool {
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

    let mode      = cv.get_u8("layout_mode");
    let (cw, ch)  = cv.canvas_size();
    let panel_top = TOPBAR_H + 1.0;
    let panel_h   = ch - panel_top;
    let a         = (cv.get_f32("ratio_a") * cw).round();

    if (mx - a).abs() <= HIT_HALF {
        cv.set_var("drag_which", quartz::Value::from(1u8));
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

pub fn on_release(cv: &mut Canvas) {
    cv.set_var("drag_which", quartz::Value::from(0u8));
}

pub fn on_move(cv: &mut Canvas, mx: f32, my: f32) {
    let which = cv.get_u8("drag_which");
    if which == 0 { return; }

    let mode         = cv.get_u8("layout_mode");
    let (cw, ch)     = cv.canvas_size();
    let panel_top    = TOPBAR_H + 1.0;
    let panel_h      = ch - panel_top;
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

pub fn update(cv: &mut Canvas) -> Panels {
    let (cw, ch)     = cv.canvas_size();
    let panel_top    = TOPBAR_H + 1.0;
    let panel_h      = ch - panel_top;
    let mode         = cv.get_u8("layout_mode");
    let min_explorer = cv.get_f32("min_explorer");

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

    let right_x = a + DIV_W;
    let right_w = cw - right_x;

    // Background
    if let Some(o) = cv.get_game_object_mut("app_bg") {
        if o.size != (cw, ch) {
            o.size = (cw, ch);
            o.set_image(quartz::tint_overlay(cw, ch, COL_APP_BG));
        }
    }

    // Topbar
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

    // Icons — reposition only; images swapped on click in on_press
    let rects = icon_rects(cw);
    if let Some(o) = cv.get_game_object_mut("icon_stacked")    { o.position = (rects[0].0, rects[0].1); }
    if let Some(o) = cv.get_game_object_mut("icon_sidebyside") { o.position = (rects[1].0, rects[1].1); }

    // Divider A
    if let Some(o) = cv.get_game_object_mut("divider_a") {
        o.position = (a, panel_top);
        if (o.size.1 - panel_h).abs() > 0.5 {
            o.size = (DIV_W, panel_h);
            o.set_image(divider_image_v(panel_h));
        }
    }

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

        Panels {
            explorer: (0.0,       panel_top, a,              panel_h),
            editor:   (right_x,   panel_top, b - right_x,    panel_h),
            terminal: (b + DIV_W, panel_top, cw - b - DIV_W, panel_h),
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
            explorer: (0.0,     panel_top, a,       panel_h),
            editor:   (right_x, panel_top, right_w, c_y - panel_top),
            terminal: (right_x, tm_y,      right_w, ch - tm_y),
        }
    }
}