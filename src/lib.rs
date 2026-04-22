pub mod constants;
pub mod logic;
pub mod objects;
pub mod preferences;
pub mod rampstack;

use crate::constants::*;
use crate::rampstack::windows::DIV_W;
use editor::prelude::{Editor, Settings as EditorSettings};
use explorer::{mount as explorer_mount, ExplorerSettings};
use flowmango::LayerId;
use flowmango::Scene;
use quartz::CanvasMode;
use quartz::{Arc, Font, Shared};
use terminal::preferences::TermSettings;
use terminal::tabbar::TAB_H;
use terminal::{mount as terminal_mount, run_command};

use ramp::prism;
use ramp::prism::Context;

pub struct App;

impl App {
    pub fn new(context: &mut Context, assets: Assets) -> Scene {
        let mut scene = Scene::new(context, CanvasMode::Fullscreen, 1);
        let layer_id = LayerId(0);

        let font_bold_b = assets
            .get_font("JetBrainsMono-Bold.ttf")
            .expect("bold font");
        let font_reg_b = assets
            .get_font("JetBrainsMono-Regular.ttf")
            .expect("regular font");
        let font_fa_b = assets.get_font("fa-solid-900.ttf").expect("fa font");

        let theme_bytes = std::fs::read("resources/cobalt.tmTheme")
            .or_else(|_| std::fs::read("resources/dark-rust.tmTheme"))
            .expect("theme file not found in resources/");

        let code_font = Arc::new(Font::from_bytes(&font_reg_b).expect("regular font"));
        let gutter_font = Arc::new(Font::from_bytes(&font_bold_b).expect("bold font"));

        // ── Settings ─────────────────────────────────────────────────────────
        logic::settings::ensure_file();

        let mut editor_settings = EditorSettings::default();
        editor_settings.backspace_deletes_before = true;
        editor_settings.auto_pairs = true;
        let mut ex_settings_init = ExplorerSettings::default();
        let mut term_settings_init = TermSettings::default();
        logic::settings::load(
            &mut editor_settings,
            &mut ex_settings_init,
            &mut term_settings_init,
        );

        // ── Layout ────────────────────────────────────────────────────────────
        let ratio_a = INIT_EXPLORER_RATIO;
        let ratio_b = 1.0 - INIT_TERMINAL_RATIO;
        let init_split_a = (INIT_CW * ratio_a).round();
        let init_split_b = (INIT_CW * ratio_b).round();
        let panel_top = preferences::TOPBAR_H + 1.0;

        let project_root = rampstack::project::resolve_project_root();
        let initial_file = rampstack::project::pick_initial_file(&project_root)
            .unwrap_or_else(|| "code.txt".to_string());

        // ── Chrome ────────────────────────────────────────────────────────────
        {
            let cv = scene.get_layer_mut(layer_id).unwrap().canvas_mut();
            objects::windows_obj::setup(cv, INIT_CW, INIT_CH, ratio_a, ratio_b);
        }

        // ── Editor ────────────────────────────────────────────────────────────
        let ed = Arc::new(Editor::new(
            init_split_a + DIV_W,
            panel_top,
            init_split_b - init_split_a - DIV_W,
            INIT_CH - panel_top,
            code_font.clone(),
            gutter_font.clone(),
            &initial_file,
            &theme_bytes,
            editor_settings,
        ));

        // ── Explorer ──────────────────────────────────────────────────────────
        ex_settings_init.x = 0.0;
        ex_settings_init.y = panel_top;
        ex_settings_init.w = init_split_a;
        ex_settings_init.project_root = project_root.clone();
        let ex_settings_shared = Shared::new(ex_settings_init);

        let ed_for_open = ed.clone();
        let on_file_open: Box<dyn Fn(&str) + 'static> = Box::new(move |path: &str| {
            ed_for_open.open_file(path);
        });

        let ex_component = explorer_mount(
            context,
            &mut scene,
            layer_id,
            font_bold_b.clone(),
            font_reg_b.clone(),
            font_fa_b,
            Some(ex_settings_shared.clone()),
            Some(on_file_open),
        );

        {
            let cv = scene.get_layer_mut(layer_id).unwrap().canvas_mut();
            let min_w = ex_settings_shared.get().min_width;
            cv.set_var("min_explorer", quartz::Value::from(min_w));
        }

        // ── Terminal ──────────────────────────────────────────────────────────
        // Pass raw panel top as offset_y — terminal::mount adds TAB_H internally.
        // The update loop writes:
        //   ts.offset_y        = p.terminal.1 + TAB_H   (terminal body)
        //   _term_panel_y var  = p.terminal.1            (tab bar)
        term_settings_init.offset_x = init_split_b + DIV_W;
        term_settings_init.offset_y = panel_top;
        let term_settings_shared = Shared::new(term_settings_init);

        let cwd = Shared::new(project_root.clone());
        let cwd_cmd = cwd.clone();
        let _term = terminal_mount(
            context,
            &mut scene,
            layer_id,
            font_reg_b,
            Some(term_settings_shared.clone()),
            move |raw_cmd, t| {
                run_command(raw_cmd, t, &cwd_cmd);
            },
        );

        // ── Mount editor ──────────────────────────────────────────────────────
        {
            let cv = scene.get_layer_mut(layer_id).unwrap().canvas_mut();
            ed.mount(cv);
            ed.register_callbacks(cv);
        }

        // ── Input handlers ────────────────────────────────────────────────────
        scene
            .get_layer_mut(layer_id)
            .unwrap()
            .canvas_mut()
            .on_mouse_press(move |cv, _btn, (mx, my)| {
                logic::windows_obj::on_press(cv, mx, my);
            });
        scene
            .get_layer_mut(layer_id)
            .unwrap()
            .canvas_mut()
            .on_mouse_release(move |cv, _btn, _pos| {
                logic::windows_obj::on_release(cv);
            });
        scene
            .get_layer_mut(layer_id)
            .unwrap()
            .canvas_mut()
            .on_mouse_move(move |cv, (mx, my)| {
                logic::windows_obj::on_move(cv, mx, my);
            });

        // ── Update loop ───────────────────────────────────────────────────────
        let ed_resize = ed.clone();
        let ex_resize = Arc::new(ex_component.resize);
        let ex_settings_upd = ex_settings_shared.clone();
        let ts_settings_upd = term_settings_shared.clone();

        scene
            .get_layer_mut(layer_id)
            .unwrap()
            .canvas_mut()
            .on_update(move |cv| {
                let (cw, ch) = cv.canvas_size();
                if cw < 1.0 || ch < 1.0 {
                    return;
                }

                let min_w = ex_settings_upd.get().min_width;
                cv.set_var("min_explorer", quartz::Value::from(min_w));

                let p = logic::windows_obj::update(cv);

                {
                    let mut es = ex_settings_upd.get_mut();
                    es.x = p.explorer.0;
                    es.y = p.explorer.1;
                    es.w = p.explorer.2;
                }
                ex_resize(cv, p.explorer.0, p.explorer.1, p.explorer.2, p.explorer.3);
                ed_resize.set_bounds(p.editor.0, p.editor.1, p.editor.2, p.editor.3);

                {
                    let mut ts = ts_settings_upd.get_mut();
                    ts.offset_x = p.terminal.0;
                    // Terminal body sits at panel top + TAB_H.
                    ts.offset_y = p.terminal.1 + TAB_H;
                }
                // Raw panel top for the tab bar.
                cv.set_var("_term_panel_y", p.terminal.1);
            });

        scene
    }
}

ramp::run! { |context: &mut Context, assets: Assets| {
    App::new(context, assets)
}}
