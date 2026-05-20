mod components;
mod git;
mod preferences;
mod project;
mod settings;
mod theme;
mod window;

use components::{
    editor::EditorComponent, explorer::ExplorerComponent, extensions::ExtensionsPanel,
    git_panel::GitPanel, liveshare::LiveSharePanel, search::SearchPanel,
    terminal::TerminalComponent,
};
use editor::prelude::Settings as EditorSettings;
use explorer::ExplorerSettings;
use flowmango::{LayerId, Scene};
use git::{lang_for_path, GitInfo};
use preferences::*;
use quartz::{Arc, CanvasMode, Font, Shared};
use ramp::prism;
use ramp::prism::Context;
use terminal::preferences::TermSettings;
use window_layout::constants::DIV_W;

pub struct App;

impl App {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(context: &mut Context) -> Scene {
        let mut scene = Scene::new(context, CanvasMode::Fullscreen, 1);
        let layer_id = LayerId(0);
        let panel_top = TOPBAR_H + 1.0;
        let init_split_a = (INIT_CW * INIT_EXPLORER_RATIO).round();
        let init_split_b = (INIT_CW * (1.0 - INIT_TERMINAL_RATIO)).round();

        let font_bold_b = include_bytes!("../resources/JetBrainsMono-Bold.ttf").to_vec();
        let font_reg_b = include_bytes!("../resources/JetBrainsMono-Regular.ttf").to_vec();
        let font_fa_b = include_bytes!("../resources/fa-solid-900.ttf").to_vec();
        let font_ph_b = include_bytes!("../resources/Phosphor-Light.ttf").to_vec();

        let code_font = Arc::new(Font::from_bytes(&font_reg_b).expect("regular font"));
        let gutter_font = Arc::new(Font::from_bytes(&font_bold_b).expect("bold font"));
        let ph_font = Arc::new(Font::from_bytes(&font_ph_b).expect("phosphor font"));

        let fallback_bytes = include_bytes!("../resources/themes/theme.json").to_vec();
        let theme_bytes: Vec<u8> =
            std::fs::read("resources/themes/jbrs.json").unwrap_or(fallback_bytes);
        let app_theme = theme::AppTheme::from_bytes(theme_bytes);

        settings::ensure_file();
        let mut ed_settings = EditorSettings::default();
        ed_settings.backspace_deletes_before = true;
        ed_settings.auto_pairs = true;
        let mut ex_settings = ExplorerSettings::default();
        let mut term_settings = TermSettings::default();
        settings::load(&mut ed_settings, &mut ex_settings, &mut term_settings);

        {
            let et = &app_theme.explorer;
            ex_settings.color_bg = et.bg.clone();
            ex_settings.color_text = et.text.clone();
            ex_settings.color_file = et.file.clone();
            ex_settings.color_crumb = et.crumb.clone();
            ex_settings.color_folder_icon = et.folder_icon.clone();
            ex_settings.color_guide = et.guide.clone();
        }

        let project_root = project::resolve_project_root();
        let initial_file =
            project::pick_initial_file(&project_root).unwrap_or_else(|| "code.txt".to_string());

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

        let editor_focus: Shared<bool> = Shared::new(true);
        let terminal_focus: Shared<bool> = Shared::new(false);

        let editor = EditorComponent::new(
            init_split_a + DIV_W,
            panel_top,
            init_split_b - init_split_a - DIV_W,
            INIT_CH - panel_top,
            code_font.clone(),
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

        ex_settings.x = SIDEBAR_W;
        ex_settings.y = panel_top;
        ex_settings.w = init_split_a - SIDEBAR_W;
        ex_settings.project_root = project_root.clone();
        let ex_settings_shared = Shared::new(ex_settings);

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
            let cv = scene.get_layer_mut(layer_id).unwrap().canvas_mut();
            let min_w = ex_settings_shared.get().min_width;
            cv.set_var("wl_min_explorer", quartz::Value::from(min_w));
        }

        // ── Side panels ───────────────────────────────────────────────────────
        let panel_w = init_split_a - SIDEBAR_W;
        let panel_h = INIT_CH - panel_top;

        let search_panel = {
            let cv = scene.get_layer_mut(layer_id).unwrap().canvas_mut();
            let p = SearchPanel::new(SIDEBAR_W, panel_top, panel_w, panel_h, code_font.clone());
            p.mount(cv);
            p.hide(cv);
            p
        };
        let git_panel = {
            let cv = scene.get_layer_mut(layer_id).unwrap().canvas_mut();
            let p = GitPanel::new(SIDEBAR_W, panel_top, panel_w, panel_h, code_font.clone());
            p.mount(cv);
            p.hide(cv);
            p
        };
        let ext_panel = {
            let cv = scene.get_layer_mut(layer_id).unwrap().canvas_mut();
            let p = ExtensionsPanel::new(SIDEBAR_W, panel_top, panel_w, panel_h, code_font.clone());
            p.mount(cv);
            p.hide(cv);
            p
        };
        let ls_panel = {
            let cv = scene.get_layer_mut(layer_id).unwrap().canvas_mut();
            let p = LiveSharePanel::new(SIDEBAR_W, panel_top, panel_w, panel_h, code_font.clone());
            p.mount(cv);
            p.hide(cv);
            p
        };

        // Last query we ran search on — avoid re-scanning on every tick
        let last_search_query: Shared<String> = Shared::new(String::new());
        let prev_sidebar: Shared<u8> = Shared::new(ICON_ID_FILES);

        term_settings.offset_x = init_split_b + DIV_W;
        term_settings.offset_y = panel_top;
        let term_settings_shared = Shared::new(term_settings);
        let cwd = Shared::new(project_root.clone());

        let _terminal = TerminalComponent::new(
            context,
            &mut scene,
            layer_id,
            font_reg_b,
            term_settings_shared.clone(),
            cwd,
            terminal_focus.clone(),
        );

        // ── Input handlers ────────────────────────────────────────────────────
        let cfg_press = wl_cfg.clone();
        let ed_press = editor.clone();
        let ef_press = editor_focus.clone();
        let tf_press = terminal_focus.clone();

        scene
            .get_layer_mut(layer_id)
            .unwrap()
            .canvas_mut()
            .on_mouse_press(move |cv, _btn, (mx, my)| {
                window::handle_press(cv, mx, my, &cfg_press);
                let (ex, ey, ew, eh) = ed_press.bounds();
                let in_editor = mx >= ex && mx <= ex + ew && my >= ey && my <= ey + eh;
                *ef_press.get_mut() = in_editor;
                *tf_press.get_mut() = !in_editor;
                ed_press.set_focus(in_editor);
            });

        let cfg_rel = wl_cfg.clone();
        scene
            .get_layer_mut(layer_id)
            .unwrap()
            .canvas_mut()
            .on_mouse_release(move |cv, _btn, _pos| {
                window::handle_release(cv, &cfg_rel);
            });

        scene
            .get_layer_mut(layer_id)
            .unwrap()
            .canvas_mut()
            .on_mouse_move(move |cv, (mx, my)| {
                window::handle_move(cv, mx, my);
            });

        // ── Update loop ───────────────────────────────────────────────────────
        let cfg_upd = wl_cfg.clone();
        let ex_share = ex_settings_shared.clone();
        let ts_share = term_settings_shared.clone();
        let ed_upd = editor.clone();
        let ef_upd = editor_focus.clone();
        let git_info_upd = git_info;
        let cur_file_u = current_file.clone();
        let prev_sb_u = prev_sidebar.clone();
        let pr_upd = project_root.clone();
        let last_q_u = last_search_query.clone();

        scene
            .get_layer_mut(layer_id)
            .unwrap()
            .canvas_mut()
            .on_update(move |cv| {
                let (cw, ch) = cv.canvas_size();
                if cw < 1.0 || ch < 1.0 {
                    return;
                }

                let min_w = ex_share.get().min_width;
                cv.set_var("wl_min_explorer", quartz::Value::from(min_w));

                let p = window::tick(cv, &cfg_upd);
                let sidebar_now = cv.get_u8("wl_sidebar_active");
                let sidebar_prev = *prev_sb_u.get();

                let px = if p.explorer_visible {
                    p.explorer.0
                } else {
                    SIDEBAR_W
                };
                let py = p.explorer.1;
                let pw = p.explorer.2;
                let ph = p.explorer.3;

                if sidebar_now != sidebar_prev {
                    match sidebar_prev {
                        x if x == ICON_ID_FILES => {
                            ex_share.get_mut().x = -9999.0;
                        }
                        x if x == ICON_ID_SEARCH => search_panel.hide(cv),
                        x if x == ICON_ID_GIT => git_panel.hide(cv),
                        x if x == ICON_ID_EXTENSIONS => ext_panel.hide(cv),
                        x if x == ICON_ID_USERS => ls_panel.hide(cv),
                        _ => {}
                    }
                    match sidebar_now {
                        x if x == ICON_ID_FILES => {
                            ex_share.get_mut().x = px;
                        }
                        x if x == ICON_ID_SEARCH => {
                            search_panel.show(cv);
                            *ef_upd.get_mut() = false;
                        }
                        x if x == ICON_ID_GIT => {
                            git_panel.refresh(&pr_upd);
                            git_panel.show(cv);
                        }
                        x if x == ICON_ID_EXTENSIONS => ext_panel.show(cv),
                        x if x == ICON_ID_USERS => ls_panel.show(cv),
                        _ => {}
                    }
                    *prev_sb_u.get_mut() = sidebar_now;
                }

                if p.explorer_visible {
                    match sidebar_now {
                        x if x == ICON_ID_FILES => {
                            let mut es = ex_share.get_mut();
                            es.x = px;
                            es.y = py;
                            es.w = pw;
                        }
                        x if x == ICON_ID_SEARCH => {
                            search_panel.resize(cv, px, py, pw, ph);

                            let q = search_panel.query.get().clone();
                            if q != *last_q_u.get() {
                                *last_q_u.get_mut() = q.clone();
                                let files = collect_project_files(&pr_upd, 300);
                                let file_refs: Vec<(&str, &str)> = files
                                    .iter()
                                    .map(|(p, c)| (p.as_str(), c.as_str()))
                                    .collect();
                                search_panel.run_search(&q, &file_refs);
                            }

                            search_panel.update(cv);
                        }
                        x if x == ICON_ID_GIT => {
                            git_panel.update(cv);
                            git_panel.resize(cv, px, py, pw, ph);
                        }
                        x if x == ICON_ID_EXTENSIONS => ext_panel.resize(cv, px, py, pw, ph),
                        x if x == ICON_ID_USERS => ls_panel.resize(cv, px, py, pw, ph),
                        _ => {}
                    }
                } else {
                    search_panel.hide(cv);
                    git_panel.hide(cv);
                    ext_panel.hide(cv);
                    ls_panel.hide(cv);
                    ex_share.get_mut().x = -9999.0;
                }

                let ex_x = ex_share.get().x;
                explorer.resize(cv, ex_x, py, pw, ph);

                ed_upd.set_bounds(p.editor.0, p.editor.1, p.editor.2, p.editor.3);
                {
                    let mut ts = ts_share.get_mut();
                    ts.offset_x = p.terminal.0;
                    ts.offset_y = p.terminal.1;
                }

                let search_focused = sidebar_now == ICON_ID_SEARCH && *search_panel.focused.get();
                if search_focused {
                    ed_upd.set_focus(false);
                } else {
                    ed_upd.set_focus(*ef_upd.get());
                }

                // ── Status bar ────────────────────────────────────────────────
                cv.set_var(
                    "wl_sb_sb_branch",
                    quartz::Value::from(git_info_upd.read_branch()),
                );
                cv.set_var(
                    "wl_sb_sb_commit",
                    quartz::Value::from(git_info_upd.read_last_commit()),
                );
                let (row, col) = ed_upd.inner.cursor_position();
                cv.set_var(
                    "wl_sb_sb_cursor",
                    quartz::Value::from(format!("Ln {}, Col {}", row + 1, col + 1)),
                );
                let lang = lang_for_path(&cur_file_u.get());
                cv.set_var("wl_sb_sb_lang", quartz::Value::from(lang.to_string()));
            });

        scene
    }
}

fn collect_project_files(root: &str, max_files: usize) -> Vec<(String, String)> {
    let mut out = Vec::new();
    collect_recursive(std::path::Path::new(root), root, &mut out, max_files);
    out
}

fn collect_recursive(
    dir: &std::path::Path,
    root: &str,
    out: &mut Vec<(String, String)>,
    max_files: usize,
) {
    if out.len() >= max_files {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        if out.len() >= max_files {
            return;
        }
        let path = entry.path();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if name_str.starts_with('.') || name_str == "target" || name_str == "node_modules" {
            continue;
        }

        if path.is_dir() {
            collect_recursive(&path, root, out, max_files);
        } else if is_text_file(&path) {
            if let Ok(content) = std::fs::read_to_string(&path) {
                let rel = path
                    .strip_prefix(root)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| path.to_string_lossy().to_string());
                out.push((rel, content));
            }
        }
    }
}

fn is_text_file(path: &std::path::Path) -> bool {
    matches!(
        path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase()
            .as_str(),
        "rs" | "toml"
            | "json"
            | "md"
            | "txt"
            | "js"
            | "ts"
            | "tsx"
            | "jsx"
            | "css"
            | "html"
            | "yml"
            | "yaml"
            | "sh"
            | "lock"
            | "gitignore"
            | "env"
            | "py"
            | "go"
            | "c"
            | "cpp"
            | "h"
            | "hpp"
            | "java"
            | "kt"
            | "swift"
            | "rb"
    )
}

ramp::run! { []; |context: &mut Context| {
    App::new(context)
}}

