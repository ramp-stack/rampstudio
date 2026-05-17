use flowmango::{LayerId, Scene};
use quartz::Shared;
use ramp::prism::Context;
use terminal::preferences::TermSettings;
use terminal::Terminal;

pub struct TerminalComponent {
    pub settings: Shared<TermSettings>,
    pub focus:    Shared<bool>,
}

impl TerminalComponent {
    pub fn new(
        context:  &mut Context,
        scene:    &mut Scene,
        layer_id: LayerId,
        font_bytes: Vec<u8>,
        settings: Shared<TermSettings>,
        cwd:      Shared<String>,
        focus:    Shared<bool>,
    ) -> Self {
        let cwd_str    = { cwd.get().clone() };
        let focus_term = focus.clone();

        terminal::mount(
            context,
            scene,
            layer_id,
            font_bytes,
            Some(settings.clone()),
            cwd_str,
            move |cmd, term| {
                terminal::run_command(cmd, term);
            },
            focus_term,
        );

        Self { settings, focus }
    }
}