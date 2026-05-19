// components/search.rs — Slick search panel

use flowmango::{Canvas, GameObject};
use quartz::{tint_overlay, Arc, Align, Color, Font, Key, NamedKey, Shared, Span, Text};

// ── Palette ───────────────────────────────────────────────────────────────────
const COL_BG:           Color = Color(22,  23,  34,  255);
const COL_HDR_BG:       Color = Color(17,  18,  27,  255);
const COL_INPUT_BG:     Color = Color(13,  14,  22,  255);
const COL_INPUT_BORDER: Color = Color(50,  54,  82,  255);
const COL_INPUT_FOCUS:  Color = Color(122, 162, 247, 255);
const COL_SEP:          Color = Color(32,  34,  52,  255);
const COL_ROW_HOVER:    Color = Color(30,  33,  50,  255);
const COL_ACCENT_BAR:   Color = Color(122, 162, 247, 255);
const COL_BTN_BG:       Color = Color(28,  31,  46,  255);
const COL_BTN_ACTIVE:   Color = Color(45,  65,  120, 255);
const COL_BTN_BORDER:   Color = Color(50,  54,  82,  255);

const COL_HDR_TITLE:    Color = Color(86,  95,  137, 255);
const COL_COUNT_FG:     Color = Color(122, 162, 247, 255);
const COL_HINT:         Color = Color(58,  65,  100, 255);
const COL_INPUT_TEXT:   Color = Color(192, 202, 245, 255);
const COL_FILE_NAME:    Color = Color(192, 202, 245, 255);
const COL_LINE_NO:      Color = Color(72,  80,  120, 255);
const COL_PREVIEW:      Color = Color(130, 140, 180, 255);
const COL_MATCH_TEXT:   Color = Color(224, 175, 104, 255);
const COL_SEC_LABEL:    Color = Color(72,  80,  120, 255);
const COL_BTN_LABEL:    Color = Color(86,  95,  137, 255);
const COL_BTN_LABEL_ON: Color = Color(192, 202, 245, 255);

// ── Layout ────────────────────────────────────────────────────────────────────
const FS:       f32 = 12.0;
const FS_SM:    f32 = 11.0;
const FS_XS:    f32 = 10.0;
const PAD:      f32 = 12.0;
const HDR_H:    f32 = 36.0;
const INPUT_H:  f32 = 32.0;
const BTN_H:    f32 = 22.0;
const BTN_W:    f32 = 28.0;
const ROW_H:    f32 = 42.0;   // taller rows — file + preview stacked
const MAX_RES:  usize = 60;

#[derive(Clone)]
pub struct SearchResult {
    pub file:    String,
    pub line:    usize,
    pub preview: String,
}

#[derive(Clone)]
pub struct SearchPanel {
    x: Shared<f32>, y: Shared<f32>,
    w: Shared<f32>, h: Shared<f32>,
    pub query:   Shared<String>,
    pub results: Shared<Vec<SearchResult>>,
    pub focused: Shared<bool>,
    font: Arc<Font>,
}

impl SearchPanel {
    pub fn new(x: f32, y: f32, w: f32, h: f32, font: Arc<Font>) -> Self {
        Self {
            x: Shared::new(x), y: Shared::new(y),
            w: Shared::new(w), h: Shared::new(h),
            query:   Shared::new(String::new()),
            results: Shared::new(Vec::new()),
            focused: Shared::new(false),
            font,
        }
    }

    pub fn mount(&self, cv: &mut Canvas) {
        let (x, y, w, h) = self.xywh();
        let iw = w - PAD * 2.0;
        let iy = y + HDR_H + 8.0;
        let btn_y = iy + INPUT_H + 6.0;
        let div_y = btn_y + BTN_H + 8.0;
        let results_y = div_y + 28.0;

        // ── BG ────────────────────────────────────────────────────────────────
        img(cv, "sp_bg",     x, y, w, h,    COL_BG,     4);
        img(cv, "sp_hdr_bg", x, y, w, HDR_H, COL_HDR_BG, 5);
        img(cv, "sp_hdr_sep",x, y + HDR_H, w, 1.0, COL_SEP, 5);

        // ── Header ────────────────────────────────────────────────────────────
        {
            let mut o = go("sp_title", x + PAD, y + (HDR_H - FS_XS) * 0.5, w - PAD*2.0, FS_XS * 1.4, 6)
                .clip().clip_origin(x, y).clip_size(w, h).finish();
            o.set_drawable(Box::new(txt("SEARCH", FS_XS, COL_HDR_TITLE, &self.font)));
            cv.add_game_object("sp_title".into(), o);
        }
        {
            let mut o = go("sp_count", x + w - 90.0, y + (HDR_H - FS_SM) * 0.5, 80.0, FS_SM * 1.4, 6)
                .clip().clip_origin(x, y).clip_size(w, h).finish();
            o.set_drawable(Box::new(txt("", FS_SM, COL_COUNT_FG, &self.font)));
            cv.add_game_object("sp_count".into(), o);
        }

        // ── Input ─────────────────────────────────────────────────────────────
        img(cv, "sp_in_bg", x + PAD, iy, iw, INPUT_H, COL_INPUT_BG, 5);
        // Border: 4 sides
        img(cv, "sp_in_t", x + PAD,          iy,                  iw,  1.0,      COL_INPUT_BORDER, 6);
        img(cv, "sp_in_b", x + PAD,          iy + INPUT_H - 1.0,  iw,  1.0,      COL_INPUT_FOCUS,  6);
        img(cv, "sp_in_l", x + PAD,          iy,                  1.0, INPUT_H,  COL_INPUT_BORDER, 6);
        img(cv, "sp_in_r", x + PAD + iw - 1.0, iy,               1.0, INPUT_H,  COL_INPUT_BORDER, 6);

        {
            let mut o = go("sp_in_val", x + PAD + 10.0, iy + (INPUT_H - FS) * 0.5,
                           iw - 12.0, FS * 1.4, 6)
                .clip().clip_origin(x + PAD, iy).clip_size(iw, INPUT_H)
                .finish();
            o.set_drawable(Box::new(txt("Search...", FS, COL_HINT, &self.font)));
            cv.add_game_object("sp_in_val".into(), o);
        }

        // Cursor (1.5px wide)
        img(cv, "sp_cursor", x + PAD + 10.0, iy + (INPUT_H - FS * 1.3) * 0.5,
            1.5, FS * 1.3, COL_INPUT_TEXT, 7);

        // ── Toggle buttons ────────────────────────────────────────────────────
        for (i, (key, lbl)) in [
            ("sp_tog_case",  "Aa"),
            ("sp_tog_word",  "W"),
            ("sp_tog_regex", ".*"),
        ].iter().enumerate() {
            let bx = x + PAD + i as f32 * (BTN_W + 4.0);
            img(cv, key, bx, btn_y, BTN_W, BTN_H, COL_BTN_BG, 5);
            // left/right/top/bot border
            img(cv, &format!("{key}_bl"), bx, btn_y, 1.0, BTN_H, COL_BTN_BORDER, 6);
            img(cv, &format!("{key}_br"), bx + BTN_W - 1.0, btn_y, 1.0, BTN_H, COL_BTN_BORDER, 6);
            img(cv, &format!("{key}_bt"), bx, btn_y, BTN_W, 1.0, COL_BTN_BORDER, 6);
            img(cv, &format!("{key}_bb"), bx, btn_y + BTN_H - 1.0, BTN_W, 1.0, COL_BTN_BORDER, 6);
            self.lbl(cv, &format!("{key}_l"), bx + BTN_W * 0.5 - FS_XS * 0.3 * lbl.len() as f32, btn_y + (BTN_H - FS_XS) * 0.5, lbl, FS_XS, COL_BTN_LABEL);
        }

        // ── Divider + section label ───────────────────────────────────────────
        img(cv, "sp_div", x, div_y, w, 1.0, COL_SEP, 5);
        {
            let mut o = go("sp_sec_lbl", x + PAD, div_y + 8.0, w - PAD*2.0, FS_XS * 1.4, 6)
                .clip().clip_origin(x, y).clip_size(w, h).finish();
            o.set_drawable(Box::new(txt("RESULTS", FS_XS, COL_SEC_LABEL, &self.font)));
            cv.add_game_object("sp_sec_lbl".into(), o);
        }

        // ── Result rows ───────────────────────────────────────────────────────
        for i in 0..MAX_RES {
            let ry = results_y + i as f32 * ROW_H;

            // Row background (for hover)
            img(cv, &format!("sp_row_bg_{i}"), x, ry, w, ROW_H - 1.0, COL_BG, 4);

            // Left accent bar
            let mut ab = go(&format!("sp_acc_{i}"), x, ry, 2.0, ROW_H - 2.0, 6)
                .image(tint_overlay(2.0, ROW_H - 2.0, COL_ACCENT_BAR)).finish();
            ab.visible = false;
            cv.add_game_object(format!("sp_acc_{i}"), ab);

            // File name + line number (top line of card)
            let fn_key = format!("sp_fn_{i}");
            let mut fo = go(&fn_key, x + PAD + 8.0, ry + 6.0, w - PAD * 2.0 - 8.0, FS_SM * 1.4, 6)
                .clip().clip_origin(x, results_y).clip_size(w, h - (results_y - y))
                .finish();
            fo.set_drawable(Box::new(txt("", FS_SM, COL_FILE_NAME, &self.font)));
            fo.visible = false;
            cv.add_game_object(fn_key, fo);

            // Preview (bottom line of card, clearly below filename)
            let pv_key = format!("sp_pv_{i}");
            let pv_y   = ry + 6.0 + FS_SM * 1.4 + 2.0;
            let mut po = go(&pv_key, x + PAD + 8.0, pv_y, w - PAD * 2.0 - 8.0, FS_SM * 1.4, 6)
                .clip().clip_origin(x, results_y).clip_size(w, h - (results_y - y))
                .finish();
            po.set_drawable(Box::new(txt("", FS_SM, COL_PREVIEW, &self.font)));
            po.visible = false;
            cv.add_game_object(pv_key, po);

            // Row bottom separator
            let sep_key = format!("sp_row_sep_{i}");
            let mut so = go(&sep_key, x + PAD, ry + ROW_H - 1.0, w - PAD, 1.0, 5)
                .image(tint_overlay(w - PAD, 1.0, COL_SEP)).finish();
            so.visible = false;
            cv.add_game_object(sep_key, so);
        }

        // ── Key handler ───────────────────────────────────────────────────────
        let query   = self.query.clone();
        let focused = self.focused.clone();
        cv.on_key_press(move |_cv, key| {
            if !*focused.get() { return; }
            match key {
                Key::Named(NamedKey::Backspace) => { query.get_mut().pop(); }
                Key::Named(NamedKey::Escape)    => { *focused.get_mut() = false; }
                Key::Named(NamedKey::Space)     => { query.get_mut().push(' '); }
                Key::Character(ch) => {
                    let c: String = ch.chars().filter(|c| !c.is_control()).collect();
                    if !c.is_empty() { query.get_mut().push_str(&c); }
                }
                _ => {}
            }
        });

        let focused2 = self.focused.clone();
        let sx = self.x.clone(); let sy2 = self.y.clone(); let sw = self.w.clone();
        cv.on_mouse_press(move |_cv, _btn, (mx, my)| {
            let (x, y, w) = (*sx.get(), *sy2.get(), *sw.get());
            let iy = y + HDR_H + 8.0;
            if mx >= x + PAD && mx <= x + w - PAD && my >= iy && my <= iy + INPUT_H {
                *focused2.get_mut() = true;
            }
        });
    }

    pub fn update(&self, cv: &mut Canvas) {
        let (x, y, w, h) = self.xywh();
        let iw        = w - PAD * 2.0;
        let iy        = y + HDR_H + 8.0;
        let btn_y     = iy + INPUT_H + 6.0;
        let div_y     = btn_y + BTN_H + 8.0;
        let results_y = div_y + 28.0;

        let query   = self.query.get().clone();
        let results = self.results.get().clone();
        let focused = *self.focused.get();

        // Input text
        if let Some(o) = cv.get_game_object_mut("sp_in_val") {
            let (t, c) = if query.is_empty() { ("Search...".into(), COL_HINT) }
                         else { (query.clone(), COL_INPUT_TEXT) };
            o.set_drawable(Box::new(txt(&t, FS, c, &self.font)));
            o.position = (x + PAD + 10.0, iy + (INPUT_H - FS) * 0.5);
            o.set_clip_origin(Some((x + PAD, iy)));
            o.set_clip_size(Some((iw, INPUT_H)));
        }

        // Focus border
        let bc = if focused { COL_INPUT_FOCUS } else { COL_INPUT_BORDER };
        for s in &["sp_in_t","sp_in_l","sp_in_r"] {
            if let Some(o) = cv.get_game_object_mut(s) {
                o.set_image(tint_overlay(o.size.0, o.size.1, bc));
            }
        }

        // Cursor
        if let Some(o) = cv.get_game_object_mut("sp_cursor") {
            let px = query.chars().count() as f32 * FS * 0.605;
            o.position = (x + PAD + 10.0 + px, iy + (INPUT_H - FS * 1.3) * 0.5);
            o.visible  = focused;
        }

        // Count
        if let Some(o) = cv.get_game_object_mut("sp_count") {
            let t = if query.is_empty() { String::new() }
                    else { format!("{} result{}", results.len(), if results.len() == 1 {""} else {"s"}) };
            o.set_drawable(Box::new(txt(&t, FS_SM, COL_COUNT_FG, &self.font)));
            o.position = (x + w - 90.0, y + (HDR_H - FS_SM) * 0.5);
        }

        // Rows
        for i in 0..MAX_RES {
            let ry  = results_y + i as f32 * ROW_H;
            let vis = ry >= results_y - ROW_H && ry < y + h;

            let fn_k  = format!("sp_fn_{i}");
            let pv_k  = format!("sp_pv_{i}");
            let ac_k  = format!("sp_acc_{i}");
            let sep_k = format!("sp_row_sep_{i}");
            let bg_k  = format!("sp_row_bg_{i}");

            if let Some(r) = results.get(i) {
                let file_short = r.file.split('/').last().unwrap_or(&r.file);
                let lineno     = format!("{}:{}", file_short, r.line);
                let preview    = r.preview.trim().to_string();
                let preview_short = if preview.len() > 80 { format!("{}…", &preview[..79]) } else { preview };

                if let Some(o) = cv.get_game_object_mut(&fn_k) {
                    o.set_drawable(Box::new(txt(&lineno, FS_SM, COL_FILE_NAME, &self.font)));
                    o.position = (x + PAD + 8.0, ry + 6.0);
                    o.set_clip_origin(Some((x, results_y)));
                    o.set_clip_size(Some((w, h - (results_y - y))));
                    o.visible = vis;
                }
                if let Some(o) = cv.get_game_object_mut(&pv_k) {
                    o.set_drawable(Box::new(txt(&preview_short, FS_SM, COL_PREVIEW, &self.font)));
                    o.position = (x + PAD + 8.0, ry + 6.0 + FS_SM * 1.4 + 2.0);
                    o.set_clip_origin(Some((x, results_y)));
                    o.set_clip_size(Some((w, h - (results_y - y))));
                    o.visible = vis;
                }
                if let Some(o) = cv.get_game_object_mut(&ac_k)  { o.position = (x, ry); o.visible = vis; }
                if let Some(o) = cv.get_game_object_mut(&sep_k) {
                    o.position = (x + PAD, ry + ROW_H - 1.0);
                    o.visible = vis;
                }
                if let Some(o) = cv.get_game_object_mut(&bg_k)  { o.position = (x, ry); o.visible = vis; }
            } else {
                for k in &[&fn_k, &pv_k, &ac_k, &sep_k, &bg_k] {
                    if let Some(o) = cv.get_game_object_mut(k) { o.visible = false; }
                }
            }
        }
    }

    pub fn resize(&self, cv: &mut Canvas, x: f32, y: f32, w: f32, h: f32) {
        *self.x.get_mut() = x; *self.y.get_mut() = y;
        *self.w.get_mut() = w; *self.h.get_mut() = h;

        let iw        = w - PAD * 2.0;
        let iy        = y + HDR_H + 8.0;
        let btn_y     = iy + INPUT_H + 6.0;
        let div_y     = btn_y + BTN_H + 8.0;
        let results_y = div_y + 28.0;

        ps(cv, "sp_bg",     x, y, w, h,    COL_BG);
        ps(cv, "sp_hdr_bg", x, y, w, HDR_H, COL_HDR_BG);
        ps(cv, "sp_hdr_sep",x, y + HDR_H, w, 1.0, COL_SEP);
        ps(cv, "sp_in_bg",  x + PAD, iy, iw, INPUT_H, COL_INPUT_BG);
        ps(cv, "sp_in_t",   x + PAD, iy, iw, 1.0, COL_INPUT_BORDER);
        ps(cv, "sp_in_b",   x + PAD, iy + INPUT_H - 1.0, iw, 1.0, COL_INPUT_FOCUS);
        ps(cv, "sp_in_l",   x + PAD, iy, 1.0, INPUT_H, COL_INPUT_BORDER);
        ps(cv, "sp_in_r",   x + PAD + iw - 1.0, iy, 1.0, INPUT_H, COL_INPUT_BORDER);
        ps(cv, "sp_div",    x, div_y, w, 1.0, COL_SEP);

        if let Some(o) = cv.get_game_object_mut("sp_title")   { o.position = (x + PAD, y + (HDR_H - FS_XS) * 0.5); o.set_clip_origin(Some((x,y))); o.set_clip_size(Some((w,h))); }
        if let Some(o) = cv.get_game_object_mut("sp_count")   { o.position = (x + w - 90.0, y + (HDR_H - FS_SM) * 0.5); o.set_clip_origin(Some((x,y))); o.set_clip_size(Some((w,h))); }
        if let Some(o) = cv.get_game_object_mut("sp_in_val")  {
            o.position = (x + PAD + 10.0, iy + (INPUT_H - FS) * 0.5);
            o.set_clip_origin(Some((x + PAD, iy)));
            o.set_clip_size(Some((iw, INPUT_H)));
        }
        if let Some(o) = cv.get_game_object_mut("sp_sec_lbl") { o.position = (x + PAD, div_y + 8.0); o.set_clip_origin(Some((x,y))); o.set_clip_size(Some((w,h))); }

        for (i, key) in ["sp_tog_case","sp_tog_word","sp_tog_regex"].iter().enumerate() {
            let bx = x + PAD + i as f32 * (BTN_W + 4.0);
            for sfx in &["", "_bl","_br","_bt","_bb"] {
                let k = format!("{key}{sfx}");
                if let Some(o) = cv.get_game_object_mut(&k) { o.position = (bx, btn_y); }
            }
            let lk = format!("{key}_l");
            if let Some(o) = cv.get_game_object_mut(&lk) { o.position = (bx + 4.0, btn_y + (BTN_H - FS_XS) * 0.5); }
        }

        for i in 0..MAX_RES {
            let ry = results_y + i as f32 * ROW_H;
            for k in &[format!("sp_row_bg_{i}"), format!("sp_acc_{i}")] {
                if let Some(o) = cv.get_game_object_mut(k) { o.position = (x, ry); }
            }
            if let Some(o) = cv.get_game_object_mut(&format!("sp_fn_{i}")) {
                o.position = (x + PAD + 8.0, ry + 6.0);
                o.set_clip_origin(Some((x, results_y)));
                o.set_clip_size(Some((w, h - (results_y - y))));
            }
            if let Some(o) = cv.get_game_object_mut(&format!("sp_pv_{i}")) {
                o.position = (x + PAD + 8.0, ry + 6.0 + FS_SM * 1.4 + 2.0);
                o.set_clip_origin(Some((x, results_y)));
                o.set_clip_size(Some((w, h - (results_y - y))));
            }
            if let Some(o) = cv.get_game_object_mut(&format!("sp_row_sep_{i}")) {
                o.position = (x + PAD, ry + ROW_H - 1.0);
            }
        }
    }

    pub fn show(&self, cv: &mut Canvas) {
        for n in &["sp_bg","sp_hdr_bg","sp_hdr_sep","sp_title",
                   "sp_in_bg","sp_in_t","sp_in_b","sp_in_l","sp_in_r","sp_in_val",
                   "sp_div","sp_sec_lbl",
                   "sp_tog_case","sp_tog_word","sp_tog_regex",
                   "sp_tog_case_l","sp_tog_word_l","sp_tog_regex_l",
                   "sp_tog_case_bl","sp_tog_case_br","sp_tog_case_bt","sp_tog_case_bb",
                   "sp_tog_word_bl","sp_tog_word_br","sp_tog_word_bt","sp_tog_word_bb",
                   "sp_tog_regex_bl","sp_tog_regex_br","sp_tog_regex_bt","sp_tog_regex_bb"] {
            if let Some(o) = cv.get_game_object_mut(n) { o.visible = true; }
        }
        *self.focused.get_mut() = true;
    }

    pub fn hide(&self, cv: &mut Canvas) {
        *self.focused.get_mut() = false;
        let always = ["sp_bg","sp_hdr_bg","sp_hdr_sep","sp_title","sp_count",
                      "sp_in_bg","sp_in_t","sp_in_b","sp_in_l","sp_in_r","sp_in_val","sp_cursor",
                      "sp_div","sp_sec_lbl",
                      "sp_tog_case","sp_tog_word","sp_tog_regex",
                      "sp_tog_case_l","sp_tog_word_l","sp_tog_regex_l",
                      "sp_tog_case_bl","sp_tog_case_br","sp_tog_case_bt","sp_tog_case_bb",
                      "sp_tog_word_bl","sp_tog_word_br","sp_tog_word_bt","sp_tog_word_bb",
                      "sp_tog_regex_bl","sp_tog_regex_br","sp_tog_regex_bt","sp_tog_regex_bb"];
        for n in &always { if let Some(o) = cv.get_game_object_mut(n) { o.visible = false; } }
        for i in 0..MAX_RES {
            for p in &["sp_fn_","sp_pv_","sp_acc_","sp_row_sep_","sp_row_bg_"] {
                if let Some(o) = cv.get_game_object_mut(&format!("{p}{i}")) { o.visible = false; }
            }
        }
    }

    pub fn run_search(&self, query: &str, files: &[(&str, &str)]) {
        let q = query.to_lowercase();
        if q.is_empty() { *self.results.get_mut() = Vec::new(); return; }
        let mut out = Vec::new();
        'outer: for (path, content) in files {
            for (li, line) in content.lines().enumerate() {
                if line.to_lowercase().contains(&q) {
                    out.push(SearchResult {
                        file:    path.to_string(),
                        line:    li + 1,
                        preview: line.to_string(),
                    });
                    if out.len() >= MAX_RES { break 'outer; }
                }
            }
        }
        *self.results.get_mut() = out;
    }

    fn xywh(&self) -> (f32, f32, f32, f32) {
        (*self.x.get(), *self.y.get(), *self.w.get(), *self.h.get())
    }
    fn lbl(&self, cv: &mut Canvas, key: &str, x: f32, y: f32, s: &str, fs: f32, col: Color) {
        let mut o = go(key, x, y, 400.0, fs * 1.4, 6).finish();
        o.set_drawable(Box::new(txt(s, fs, col, &self.font)));
        cv.add_game_object(key.into(), o);
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn go(key: &str, x: f32, y: f32, w: f32, h: f32, layer: i32) -> flowmango::GameObjectBuilder {
    GameObject::build(key).position(x, y).size(w, h).layer(layer)
}

fn img(cv: &mut Canvas, key: &str, x: f32, y: f32, w: f32, h: f32, col: Color, layer: i32) {
    cv.add_game_object(key.into(),
        go(key, x, y, w, h, layer).image(tint_overlay(w, h, col)).finish());
}

fn ps(cv: &mut Canvas, key: &str, x: f32, y: f32, w: f32, h: f32, col: Color) {
    if let Some(o) = cv.get_game_object_mut(key) {
        o.position = (x, y);
        if (o.size.0 - w).abs() > 0.5 || (o.size.1 - h).abs() > 0.5 {
            o.size = (w, h);
            o.set_image(tint_overlay(w, h, col));
        }
    }
}

fn txt(s: &str, fs: f32, col: Color, font: &Arc<Font>) -> Text {
    Text::new(
        vec![Span::new(s.to_string(), fs, Some(fs * 1.4), font.clone(), col, 0.0)],
        None, Align::Left, None,
    )
}