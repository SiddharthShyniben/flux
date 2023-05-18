use serde_derive::{Deserialize, Serialize};
use tui::widgets::ListState;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub urls: Vec<String>,
}

#[derive(Default)]
pub enum AppMode {
    #[default]
    Normal,
    Help,
    Input,
}

#[derive(Default)]
pub struct AppState {
    pub mode: AppMode,
    pub input_text: String,
    pub list_state: ListState,
}
