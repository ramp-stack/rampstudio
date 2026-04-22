use editor::prelude::Settings as EditorSettings;
use explorer::ExplorerSettings;
use terminal::preferences::TermSettings;
use crate::rampstack::settings as rs;

pub fn ensure_file() {
    rs::ensure_file();
}

pub fn load(
    ed:   &mut EditorSettings,
    ex:   &mut ExplorerSettings,
    term: &mut TermSettings,
) {
    rs::load(ed, ex, term);
}

pub fn save(
    ed:   &EditorSettings,
    ex:   &ExplorerSettings,
    term: &TermSettings,
) {
    rs::save(ed, ex, term);
}