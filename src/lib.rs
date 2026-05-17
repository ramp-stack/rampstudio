mod preferences;
mod settings;
mod project;
mod window;
mod components;
mod git;
mod theme;

use components::{
    editor::EditorComponent,
    explorer::ExplorerComponent,
    terminal::TerminalComponent,
};
use editor::prelude::Settings as EditorSettings;
use explorer::ExplorerSettings;
use flowmango::{LayerId, Scene};
use quartz::{Arc, CanvasMode, Font, Shared};
use ramp::prism;
use ramp::prism::Context;
use terminal::preferences::TermSettings;
use window_layout::constants::DIV_W;
use preferences::*;
use git::{GitInfo, lang_for_path};

pub struct App;

impl App {
    pub fn new(context: &mut Context) -> Scene {
        let mut scene    = Scene::new(context, CanvasMode::Fullscreen, 1);
        let layer_id     = LayerId(0);
        let panel_top    = TOPBAR_H + 1.0;
        let init_split_a = (INIT_CW * INIT_EXPLORER_RATIO).round();
        let init_split_b = (INIT_CW * (1.0 - INIT_TERMINAL_RATIO)).round();

        let font_bold_b = include_bytes!("../resources/JetBrainsMono-Bold.ttf").to_vec();
        let font_reg_b  = include_bytes!("../resources/JetBrainsMono-Regular.ttf").to_vec();
        let font_fa_b   = include_bytes!("../resources/fa-solid-900.ttf").to_vec();
        let font_ph_b   = include_bytes!("../resources/Phosphor-Light.ttf").to_vec();

        let code_font   = Arc::new(Font::from_bytes(&font_reg_b).expect("regular font"));
        let gutter_font = Arc::new(Font::from_bytes(&font_bold_b).expect("bold font"));
        let ph_font     = Arc::new(Font::from_bytes(&font_ph_b).expect("phosphor font"));

        // ── Theme ─────────────────────────────────────────────────────────────
        let fallback_bytes = include_bytes!("../resources/themes/theme.json").to_vec();
        let theme_bytes: Vec<u8> = std::fs::read("resources/themes/theme.json")
            .unwrap_or(fallback_bytes);
        let app_theme = theme::AppTheme::from_bytes(theme_bytes);

        settings::ensure_file();
        let mut ed_settings   = EditorSettings::default();
        ed_settings.backspace_deletes_before = true;
        ed_settings.auto_pairs               = true;
        let mut ex_settings   = ExplorerSettings::default();
        let mut term_settings = TermSettings::default();
        settings::load(&mut ed_settings, &mut ex_settings, &mut term_settings);

        // ── Apply explorer theme ──────────────────────────────────────────────
        // Override the explorer color strings from theme.json.
        // These take priority over anything in settings.json.
        {
            let et = &app_theme.explorer;
            ex_settings.color_bg          = et.bg.clone();
            ex_settings.color_text        = et.text.clone();
            ex_settings.color_file        = et.file.clone();
            ex_settings.color_crumb       = et.crumb.clone();
            ex_settings.color_folder_icon = et.folder_icon.clone();
            ex_settings.color_guide       = et.guide.clone();
        }

        let project_root = project::resolve_project_root();
        let initial_file = project::pick_initial_file(&project_root)
            .unwrap_or_else(|| "code.txt".to_string());

        let git_info = GitInfo::new();
        if !project_root.is_empty() {
            git_info.start_polling(project_root.clone(), 5);
        }

        let current_file: Shared<String> = Shared::new(initial_file.clone());

        let wl_cfg = Arc::new(window::build_config(
            ph_font,
            code_font.clone(),
            &app_theme.chrome,
        ));
        {
            let cv = scene.get_layer_mut(layer_id).unwrap().canvas_mut();
            window::setup(cv, INIT_CW, INIT_CH, &wl_cfg);
        }

        let editor_focus:   Shared<bool> = Shared::new(true);
        let terminal_focus: Shared<bool> = Shared::new(false);

        let editor = EditorComponent::new(
            init_split_a + DIV_W,
            panel_top,
            init_split_b - init_split_a - DIV_W,
            INIT_CH - panel_top,
            code_font,
            gutter_font,
            &initial_file,
            &app_theme.raw_bytes,
            ed_settings,
        );
        {
            let cv = scene.get_layer_mut(layer_id).unwrap().canvas_mut();
            editor.mount(cv);
        }
        editor.set_focus(true);

        ex_settings.x            = SIDEBAR_W;
        ex_settings.y            = panel_top;
        ex_settings.w            = init_split_a - SIDEBAR_W;
        ex_settings.project_root = project_root.clone();
        let ex_settings_shared   = Shared::new(ex_settings);

        let ed_for_open = editor.inner.clone();
        let ef_for_open = editor_focus.clone();
        let tf_for_open = terminal_focus.clone();
        let cf_for_open = current_file.clone();
        let explorer = ExplorerComponent::new(
            context,
            &mut scene,
            layer_id,
            font_bold_b,
            font_reg_b.clone(),
            font_fa_b,
            ex_settings_shared.clone(),
            Box::new(move |path: &str| {
                ed_for_open.open_file(path);
                *ef_for_open.get_mut() = true;
                *tf_for_open.get_mut() = false;
                *cf_for_open.get_mut() = path.to_string();
            }),
        );

        {
            let cv    = scene.get_layer_mut(layer_id).unwrap().canvas_mut();
            let min_w = ex_settings_shared.get().min_width;
            cv.set_var("wl_min_explorer", quartz::Value::from(min_w));
        }

        term_settings.offset_x = init_split_b + DIV_W;
        term_settings.offset_y = panel_top;
        let term_settings_shared = Shared::new(term_settings);
        let cwd = Shared::new(project_root);

        let _terminal = TerminalComponent::new(
            context,
            &mut scene,
            layer_id,
            font_reg_b,
            term_settings_shared.clone(),
            cwd,
            terminal_focus.clone(),
        );

        let cfg_press = wl_cfg.clone();
        let ed_press  = editor.clone();
        let ef_press  = editor_focus.clone();
        let tf_press  = terminal_focus.clone();

        scene.get_layer_mut(layer_id).unwrap().canvas_mut()
            .on_mouse_press(move |cv, _btn, (mx, my)| {
                window::handle_press(cv, mx, my, &cfg_press);
                let (ex, ey, ew, eh) = ed_press.bounds();
                let in_editor = mx >= ex && mx <= ex + ew && my >= ey && my <= ey + eh;
                *ef_press.get_mut() = in_editor;
                *tf_press.get_mut() = !in_editor;
                ed_press.set_focus(in_editor);
            });

        let cfg_rel = wl_cfg.clone();
        scene.get_layer_mut(layer_id).unwrap().canvas_mut()
            .on_mouse_release(move |cv, _btn, _pos| {
                window::handle_release(cv, &cfg_rel);
            });

        scene.get_layer_mut(layer_id).unwrap().canvas_mut()
            .on_mouse_move(move |cv, (mx, my)| {
                window::handle_move(cv, mx, my);
            });

        let cfg_upd        = wl_cfg.clone();
        let ex_share       = ex_settings_shared.clone();
        let ts_share       = term_settings_shared.clone();
        let ed_upd         = editor.clone();
        let ef_upd         = editor_focus.clone();
        let git_info_upd   = git_info;
        let current_file_u = current_file.clone();

        scene.get_layer_mut(layer_id).unwrap().canvas_mut()
            .on_update(move |cv| {
                let (cw, ch) = cv.canvas_size();
                if cw < 1.0 || ch < 1.0 { return; }

                let min_w = ex_share.get().min_width;
                cv.set_var("wl_min_explorer", quartz::Value::from(min_w));

                let p = window::tick(cv, &cfg_upd);

                let ex_x = if p.explorer_visible { p.explorer.0 } else { -9999.0 };
                {
                    let mut es = ex_share.get_mut();
                    es.x = ex_x;
                    es.y = p.explorer.1;
                    es.w = p.explorer.2;
                }
                explorer.resize(cv, ex_x, p.explorer.1, p.explorer.2, p.explorer.3);
                ed_upd.set_bounds(p.editor.0, p.editor.1, p.editor.2, p.editor.3);
                {
                    let mut ts = ts_share.get_mut();
                    ts.offset_x = p.terminal.0;
                    ts.offset_y = p.terminal.1;
                }
                ed_upd.set_focus(*ef_upd.get());

                cv.set_var("wl_sb_sb_branch",
                    quartz::Value::from(git_info_upd.read_branch()));
                cv.set_var("wl_sb_sb_commit",
                    quartz::Value::from(git_info_upd.read_last_commit()));

                let (row, col) = ed_upd.inner.cursor_position();
                cv.set_var("wl_sb_sb_cursor",
                    quartz::Value::from(format!("Ln {}, Col {}", row + 1, col + 1)));

                let lang = lang_for_path(&current_file_u.get());
                cv.set_var("wl_sb_sb_lang",
                    quartz::Value::from(lang.to_string()));
            });

        scene
    }
}

ramp::run! { []; |context: &mut Context| {
    App::new(context)
}}