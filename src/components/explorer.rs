use quartz::{Arc, Canvas, Shared};
use explorer::{mount as explorer_mount, ExplorerSettings};
use flowmango::{LayerId, Scene};
use ramp::prism::Context;

#[derive(Clone)]
pub struct ExplorerComponent {
    pub settings: Shared<ExplorerSettings>,
    resize_fn:    Arc<dyn Fn(&mut Canvas, f32, f32, f32, f32)>,
}

impl ExplorerComponent {
    pub fn new(
        context:      &mut Context,
        scene:        &mut Scene,
        layer_id:     LayerId,
        font_bold_b:  Vec<u8>,
        font_reg_b:   Vec<u8>,
        font_fa_b:    Vec<u8>,
        settings:     Shared<ExplorerSettings>,
        on_file_open: Box<dyn Fn(&str) + 'static>,
    ) -> Self {
        let component = explorer_mount(
            context,
            scene,
            layer_id,
            font_bold_b,
            font_reg_b,
            font_fa_b,
            Some(settings.clone()),
            Some(on_file_open),
        );
        Self {
            settings,
            resize_fn: Arc::from(component.resize),
        }
    }

    pub fn resize(&self, cv: &mut Canvas, x: f32, y: f32, w: f32, h: f32) {
        (self.resize_fn)(cv, x, y, w, h);
    }
}