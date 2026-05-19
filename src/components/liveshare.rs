// components/liveshare.rs — Live Share panel.

use flowmango::{Canvas, GameObject};
use quartz::{tint_overlay, Arc, Align, Color, Font, Shared, Span, Text};

const COL_BG:      Color = Color(26,  27,  38,  255);
const COL_DIM:     Color = Color(86,  95,  137, 255);
const COL_LABEL:   Color = Color(169, 177, 214, 255);
const COL_NAME:    Color = Color(192, 202, 245, 255);
const COL_GREEN:   Color = Color(158, 206, 106, 255);
const COL_BTN_BG:  Color = Color(122, 162, 247, 255);
const COL_BTN_TXT: Color = Color(26,  27,  38,  255);
const COL_CARD_BG: Color = Color(31,  32,  48,  255);

const FS:    f32 = 12.0;
const FS_SM: f32 = 11.0;
const PAD:   f32 = 12.0;

#[derive(Clone)]
pub struct LiveSharePanel {
    x: Shared<f32>, y: Shared<f32>,
    w: Shared<f32>, h: Shared<f32>,
    pub session_active: Shared<bool>,
    pub participants:   Shared<Vec<String>>,
    font: Arc<Font>,
}

impl LiveSharePanel {
    pub fn new(x: f32, y: f32, w: f32, h: f32, font: Arc<Font>) -> Self {
        Self {
            x: Shared::new(x), y: Shared::new(y),
            w: Shared::new(w), h: Shared::new(h),
            session_active: Shared::new(false),
            participants:   Shared::new(Vec::new()),
            font,
        }
    }

    pub fn mount(&self, cv: &mut Canvas) {
        let (x, y, w, h) = self.bounds();

        cv.add_game_object("ls_bg".into(),
            GameObject::build("ls_bg").position(x, y).size(w, h).layer(4)
                .image(tint_overlay(w, h, COL_BG)).finish());

        self.lbl(cv, "ls_header", x + PAD, y + 10.0, "LIVE SHARE", FS_SM, COL_DIM);

        // Status card
        cv.add_game_object("ls_card".into(),
            GameObject::build("ls_card").position(x + PAD * 0.5, y + 36.0)
                .size(w - PAD, 64.0).layer(5)
                .image(tint_overlay(w - PAD, 64.0, COL_CARD_BG)).finish());

        self.lbl(cv, "ls_status",   x + PAD, y + 44.0, "Not in a session", FS, COL_DIM);
        self.lbl(cv, "ls_start_btn",x + PAD, y + 64.0, "Start Collaboration Session", FS, COL_BTN_BG);

        // Participants section
        self.lbl(cv, "ls_part_hdr", x + PAD, y + 118.0, "PARTICIPANTS", FS_SM, COL_DIM);

        for i in 0..8usize {
            let n = format!("ls_participant_{i}");
            let mut o = GameObject::build(&n)
                .position(x + PAD, y + 140.0 + i as f32 * 24.0)
                .size(w - PAD * 2.0, 20.0).layer(5).finish();
            o.set_drawable(Box::new(self.text("", FS_SM, COL_DIM)));
            o.visible = false;
            cv.add_game_object(n, o);
        }

        // Shared terminals section
        self.lbl(cv, "ls_term_hdr", x + PAD, y + 148.0 + 8.0 * 24.0, "SHARED TERMINALS", FS_SM, COL_DIM);
        self.lbl(cv, "ls_term_none", x + PAD, y + 168.0 + 8.0 * 24.0, "No shared terminals", FS_SM, COL_DIM);
    }

    pub fn update(&self, cv: &mut Canvas) {
        let (x, y, w, _) = self.bounds();
        let active       = *self.session_active.get();
        let participants = self.participants.get().clone();

        if let Some(o) = cv.get_game_object_mut("ls_status") {
            let (txt, col) = if active {
                (format!(" Sharing — {} participant{}", participants.len(),
                    if participants.len() == 1 { "" } else { "s" }), COL_GREEN)
            } else {
                ("Not in a session".into(), COL_DIM)
            };
            o.set_drawable(Box::new(self.text(&txt, FS, col)));
        }

        if let Some(o) = cv.get_game_object_mut("ls_start_btn") {
            let (txt, col) = if active {
                ("  End Session", COL_DIM)
            } else {
                ("  Start Collaboration Session", COL_BTN_BG)
            };
            o.set_drawable(Box::new(self.text(txt, FS, col)));
        }

        for i in 0..8usize {
            let n = format!("ls_participant_{i}");
            if let Some(o) = cv.get_game_object_mut(&n) {
                if let Some(name) = participants.get(i) {
                    let txt = format!(" {} {}", if i == 0 { "" } else { "" }, name);
                    o.set_drawable(Box::new(self.text(&txt, FS_SM, COL_NAME)));
                    o.visible = true;
                } else {
                    o.visible = false;
                }
            }
        }

        // Background resize
        if let Some(o) = cv.get_game_object_mut("ls_bg") {
            o.position = (x, y);
        }
    }

    pub fn resize(&self, cv: &mut Canvas, x: f32, y: f32, w: f32, h: f32) {
        *self.x.get_mut() = x; *self.y.get_mut() = y;
        *self.w.get_mut() = w; *self.h.get_mut() = h;
        self.update(cv);
    }

    pub fn show(&self, cv: &mut Canvas) {
        for n in &["ls_bg","ls_header","ls_card","ls_status","ls_start_btn","ls_part_hdr","ls_term_hdr","ls_term_none"] {
            if let Some(o) = cv.get_game_object_mut(n) { o.visible = true; }
        }
    }

    pub fn hide(&self, cv: &mut Canvas) {
        for n in &["ls_bg","ls_header","ls_card","ls_status","ls_start_btn","ls_part_hdr","ls_term_hdr","ls_term_none"] {
            if let Some(o) = cv.get_game_object_mut(n) { o.visible = false; }
        }
        for i in 0..8usize {
            let n = format!("ls_participant_{i}");
            if let Some(o) = cv.get_game_object_mut(&n) { o.visible = false; }
        }
    }

    fn bounds(&self) -> (f32, f32, f32, f32) {
        (*self.x.get(), *self.y.get(), *self.w.get(), *self.h.get())
    }

    fn lbl(&self, cv: &mut Canvas, key: &str, x: f32, y: f32, txt: &str, fs: f32, col: Color) {
        let mut o = GameObject::build(key).position(x, y).size(300.0, fs * 1.4).layer(6).finish();
        o.set_drawable(Box::new(self.text(txt, fs, col)));
        cv.add_game_object(key.into(), o);
    }

    fn text(&self, s: &str, fs: f32, color: Color) -> Text {
        Text::new(vec![Span::new(s.to_string(), fs, Some(fs * 1.4), self.font.clone(), color, 0.0)],
            None, Align::Left, None)
    }
}