use quartz::{load_image_sized, tint_overlay};
use crate::preferences::*;
use crate::rampstack::windows::*;
use flowmango::GameObject;
use flowmango::Canvas;

pub fn setup(cv: &mut Canvas, init_cw: f32, init_ch: f32, ratio_a: f32, ratio_b: f32) {
    let split_a   = (init_cw * ratio_a).round();
    let split_b   = (init_cw * ratio_b).round();
    let panel_top = TOPBAR_H + 1.0;
    let panel_h   = init_ch - panel_top;

    // Background
    cv.add_game_object("app_bg".into(), GameObject::build("app_bg")
        .position(0.0, 0.0).size(4000.0, 4000.0).layer(-1)
        .image(tint_overlay(4000.0, 4000.0, COL_APP_BG)).finish());

    // Topbar
    cv.add_game_object("topbar_bg".into(), GameObject::build("topbar_bg")
        .position(0.0, 0.0).size(init_cw, TOPBAR_H).layer(5)
        .image(tint_overlay(init_cw, TOPBAR_H, COL_TOPBAR_BG)).finish());

    cv.add_game_object("topbar_sep".into(), GameObject::build("topbar_sep")
        .position(0.0, TOPBAR_H).size(init_cw, 1.0).layer(6)
        .image(tint_overlay(init_cw, 1.0, COL_BORDER)).finish());

    // Logo
    let logo_img = load_image_sized("resources/rampstacklogo.png", LOGO_W, LOGO_H);
    let logo_y   = (TOPBAR_H - LOGO_H) * 0.5;
    cv.add_game_object("topbar_logo".into(), GameObject::build("topbar_logo")
        .position(LOGO_PAD, logo_y).size(LOGO_W, LOGO_H).layer(7)
        .image(logo_img).finish());

    // Layout-mode icons
    let rects = icon_rects(init_cw);
    cv.add_game_object("icon_stacked".into(), GameObject::build("icon_stacked")
        .position(rects[0].0, rects[0].1).size(ICON_SIZE, ICON_SIZE).layer(7)
        .image(load_image_sized("resources/unselected_stacked.png", ICON_SIZE, ICON_SIZE))
        .finish());
    cv.add_game_object("icon_sidebyside".into(), GameObject::build("icon_sidebyside")
        .position(rects[1].0, rects[1].1).size(ICON_SIZE, ICON_SIZE).layer(7)
        .image(load_image_sized("resources/selected_sidebyside.png", ICON_SIZE, ICON_SIZE))
        .finish());

    // Divider A (vertical, between explorer and editor)
    let mut div_a = GameObject::build("divider_a")
        .position(split_a, panel_top).size(DIV_W, panel_h).layer(10).finish();
    div_a.set_image(divider_image_v(panel_h));
    cv.add_game_object("divider_a".into(), div_a);

    // Divider B (vertical, between editor and terminal — side-by-side mode)
    let mut div_b = GameObject::build("divider_b")
        .position(split_b, panel_top).size(DIV_W, panel_h).layer(10).finish();
    div_b.set_image(divider_image_v(panel_h));
    cv.add_game_object("divider_b".into(), div_b);

    // Divider C (horizontal, between editor and terminal — stacked mode)
    let right_w      = init_cw - split_a - DIV_W;
    let init_ratio_c = 0.6f32;
    let split_c_y    = panel_top + (panel_h * init_ratio_c).round();
    let mut div_c = GameObject::build("divider_c")
        .position(split_a + DIV_W, split_c_y).size(right_w, DIV_W).layer(10).finish();
    div_c.set_image(divider_image_h(right_w));
    div_c.visible = false;
    cv.add_game_object("divider_c".into(), div_c);

    // Canvas vars
    cv.set_var("ratio_a",      quartz::Value::from(ratio_a));
    cv.set_var("ratio_b",      quartz::Value::from(ratio_b));
    cv.set_var("ratio_c",      quartz::Value::from(init_ratio_c));
    cv.set_var("drag_which",   quartz::Value::from(0u8));
    cv.set_var("layout_mode",  quartz::Value::from(0u8));
    cv.set_var("min_explorer", quartz::Value::from(MIN_EXPLORER));
}