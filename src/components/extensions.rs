// components/extensions.rs — Extensions panel.

use flowmango::{Canvas, GameObject};
use quartz::{tint_overlay, Arc, Align, Color, Font, Shared, Span, Text};

const COL_BG:     Color = Color(26,  27,  38,  255);
const COL_LABEL:  Color = Color(169, 177, 214, 255);
const COL_DIM:    Color = Color(86,  95,  137, 255);
const COL_NAME:   Color = Color(192, 202, 245, 255);
const COL_VER:    Color = Color(86,  95,  137, 255);
const COL_BTN:    Color = Color(122, 162, 247, 255);
const COL_CARD_BG:Color = Color(31,  32,  48,  255);
const COL_SEP:    Color = Color(42,  43,  61,  255);

const FS:    f32 = 12.0;
const FS_SM: f32 = 11.0;
const PAD:   f32 = 12.0;
const CARD_H:f32 = 56.0;

pub struct Extension {
    pub name:        &'static str,
    pub description: &'static str,
    pub version:     &'static str,
    pub installed:   bool,
}

const EXTENSIONS: &[Extension] = &[
    Extension { name: "rust-analyzer",   description: "Rust language support",        version: "0.3.1",  installed: true  },
    Extension { name: "GitLens",         description: "Git supercharged",             version: "14.9.0", installed: true  },
    Extension { name: "Prettier",        description: "Code formatter",               version: "10.4.0", installed: false },
    Extension { name: "Error Lens",      description: "Inline error highlighting",    version: "3.16.0", installed: false },
    Extension { name: "Tokyo Night",     description: "Clean dark theme",             version: "1.0.7",  installed: true  },
    Extension { name: "Even Better TOML",description: "TOML language support",        version: "0.19.2", installed: false },
];

#[derive(Clone)]
pub struct ExtensionsPanel {
    x: Shared<f32>, y: Shared<f32>,
    w: Shared<f32>, h: Shared<f32>,
    font: Arc<Font>,
}

impl ExtensionsPanel {
    pub fn new(x: f32, y: f32, w: f32, h: f32, font: Arc<Font>) -> Self {
        Self {
            x: Shared::new(x), y: Shared::new(y),
            w: Shared::new(w), h: Shared::new(h),
            font,
        }
    }

    pub fn mount(&self, cv: &mut Canvas) {
        let (x, y, w, h) = self.bounds();

        cv.add_game_object("ext_bg".into(),
            GameObject::build("ext_bg").position(x, y).size(w, h).layer(4)
                .image(tint_overlay(w, h, COL_BG)).finish());

        self.spawn_label(cv, "ext_header", x + PAD, y + 10.0, "EXTENSIONS", FS_SM, COL_DIM);

        // Search box
        cv.add_game_object("ext_search_bg".into(),
            GameObject::build("ext_search_bg").position(x + PAD, y + 32.0)
                .size(w - PAD * 2.0, 26.0).layer(5)
                .image(tint_overlay(w - PAD * 2.0, 26.0, COL_CARD_BG)).finish());
        self.spawn_label(cv, "ext_search_text", x + PAD + 6.0, y + 32.0 + 7.0, "Search Extensions...", FS, COL_DIM);

        // Separator
        cv.add_game_object("ext_sep".into(),
            GameObject::build("ext_sep").position(x, y + 66.0).size(w, 1.0).layer(5)
                .image(tint_overlay(w, 1.0, COL_SEP)).finish());

        self.spawn_label(cv, "ext_installed_hdr", x + PAD, y + 74.0, "INSTALLED", FS_SM, COL_DIM);

        // Extension cards
        for (i, ext) in EXTENSIONS.iter().enumerate() {
            let card_y  = y + 94.0 + i as f32 * (CARD_H + 4.0);
            let card_bg = format!("ext_card_bg_{i}");
            let name_k  = format!("ext_name_{i}");
            let desc_k  = format!("ext_desc_{i}");
            let ver_k   = format!("ext_ver_{i}");
            let btn_k   = format!("ext_btn_{i}");

            cv.add_game_object(card_bg.clone(),
                GameObject::build(&card_bg).position(x + PAD * 0.5, card_y)
                    .size(w - PAD, CARD_H).layer(5)
                    .image(tint_overlay(w - PAD, CARD_H, COL_CARD_BG)).finish());

            self.spawn_label(cv, &name_k,  x + PAD, card_y + 8.0,  ext.name,        FS,    COL_NAME);
            self.spawn_label(cv, &desc_k,  x + PAD, card_y + 24.0, ext.description, FS_SM, COL_DIM);
            self.spawn_label(cv, &ver_k,   x + PAD, card_y + 40.0, ext.version,     FS_SM, COL_VER);

            let btn_label = if ext.installed { "Installed" } else { "Install" };
            let btn_col   = if ext.installed { COL_DIM } else { COL_BTN };
            self.spawn_label(cv, &btn_k, x + w - PAD * 4.0, card_y + 20.0, btn_label, FS_SM, btn_col);
        }
    }

    pub fn resize(&self, cv: &mut Canvas, x: f32, y: f32, w: f32, h: f32) {
        *self.x.get_mut() = x; *self.y.get_mut() = y;
        *self.w.get_mut() = w; *self.h.get_mut() = h;

        if let Some(o) = cv.get_game_object_mut("ext_bg") {
            o.position = (x, y);
            if (o.size.0 - w).abs() > 0.5 { o.size = (w, h); o.set_image(tint_overlay(w, h, COL_BG)); }
        }
        if let Some(o) = cv.get_game_object_mut("ext_header")      { o.position = (x + PAD, y + 10.0); }
        if let Some(o) = cv.get_game_object_mut("ext_search_bg")   { o.position = (x + PAD, y + 32.0); }
        if let Some(o) = cv.get_game_object_mut("ext_search_text") { o.position = (x + PAD + 6.0, y + 39.0); }
        if let Some(o) = cv.get_game_object_mut("ext_sep")         { o.position = (x, y + 66.0); }
        if let Some(o) = cv.get_game_object_mut("ext_installed_hdr"){ o.position = (x + PAD, y + 74.0); }

        for (i, _) in EXTENSIONS.iter().enumerate() {
            let card_y = y + 94.0 + i as f32 * (CARD_H + 4.0);
            let names  = [
                format!("ext_card_bg_{i}"), format!("ext_name_{i}"),
                format!("ext_desc_{i}"),    format!("ext_ver_{i}"),
            ];
            let offsets: [(f32, f32); 4] = [
                (x + PAD * 0.5, card_y), (x + PAD, card_y + 8.0),
                (x + PAD, card_y + 24.0), (x + PAD, card_y + 40.0),
            ];
            for (k, n) in names.iter().enumerate() {
                if let Some(o) = cv.get_game_object_mut(n) { o.position = offsets[k]; }
            }
            if let Some(o) = cv.get_game_object_mut(&format!("ext_btn_{i}")) {
                o.position = (x + w - PAD * 4.0, card_y + 20.0);
            }
        }
    }

    pub fn show(&self, cv: &mut Canvas) {
        let names: Vec<String> = vec![
            "ext_bg".into(), "ext_header".into(), "ext_search_bg".into(),
            "ext_search_text".into(), "ext_sep".into(), "ext_installed_hdr".into(),
        ];
        for n in &names { if let Some(o) = cv.get_game_object_mut(n) { o.visible = true; } }
        for i in 0..EXTENSIONS.len() {
            for pfx in &["ext_card_bg_","ext_name_","ext_desc_","ext_ver_","ext_btn_"] {
                let n = format!("{pfx}{i}");
                if let Some(o) = cv.get_game_object_mut(&n) { o.visible = true; }
            }
        }
    }

    pub fn hide(&self, cv: &mut Canvas) {
        let names: Vec<String> = vec![
            "ext_bg".into(), "ext_header".into(), "ext_search_bg".into(),
            "ext_search_text".into(), "ext_sep".into(), "ext_installed_hdr".into(),
        ];
        for n in &names { if let Some(o) = cv.get_game_object_mut(n) { o.visible = false; } }
        for i in 0..EXTENSIONS.len() {
            for pfx in &["ext_card_bg_","ext_name_","ext_desc_","ext_ver_","ext_btn_"] {
                let n = format!("{pfx}{i}");
                if let Some(o) = cv.get_game_object_mut(&n) { o.visible = false; }
            }
        }
    }

    fn bounds(&self) -> (f32, f32, f32, f32) {
        (*self.x.get(), *self.y.get(), *self.w.get(), *self.h.get())
    }

    fn spawn_label(&self, cv: &mut Canvas, key: &str, x: f32, y: f32, text: &str, fs: f32, col: Color) {
        let mut o = GameObject::build(key).position(x, y).size(300.0, fs * 1.4).layer(6).finish();
        o.set_drawable(Box::new(Text::new(
            vec![Span::new(text.to_string(), fs, Some(fs * 1.4), self.font.clone(), col, 0.0)],
            None, Align::Left, None,
        )));
        cv.add_game_object(key.into(), o);
    }
}