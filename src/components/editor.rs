use editor::prelude::Settings;
use editor::Editor;
use flowmango::Canvas;
use quartz::{Arc, Font};
use std::sync::Mutex;

#[derive(Clone)]
pub struct EditorComponent {
    pub inner: Arc<Mutex<Editor>>,
}

impl EditorComponent {
    pub fn new(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        code_font: Arc<Font>,
        gutter_font: Arc<Font>,
        initial_file: &str,
        theme_bytes: &[u8],
        settings: Settings,
    ) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Editor::new(
                x,
                y,
                w,
                h,
                code_font,
                gutter_font,
                initial_file,
                theme_bytes,
                settings,
            ))),
        }
    }

    pub fn mount(&self, cv: &mut Canvas) {
        self.inner.lock().unwrap().mount(cv);
        self.inner.lock().unwrap().register_callbacks(cv);
    }

    pub fn set_bounds(&self, x: f32, y: f32, w: f32, h: f32) {
        self.inner.lock().unwrap().set_bounds(x, y, w, h);
    }

    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        self.inner.lock().unwrap().get_bounds()
    }

    pub fn set_focus(&self, focused: bool) {
        let mut editor = self.inner.lock().unwrap();
        editor.set_focus(focused);
    }
}

