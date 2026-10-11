pub mod document;
pub mod home;
pub mod settings;

pub use document::DocumentContent;
pub use home::{HomeAction, HomeView};
pub use settings::{render_settings, SettingsAction, SettingsState};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AppView {
    Home,
    Document {
        title: String,
        path: Option<String>,
    },
    Settings,
}
