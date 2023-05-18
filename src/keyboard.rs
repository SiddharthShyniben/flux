use crossterm::event::{KeyCode, KeyEvent};
use crate::models::{AppState, AppMode, Config};

pub fn handle_event(config: &mut Config, state: &mut AppState, key: KeyEvent) -> bool {
    match state.mode {
        AppMode::Input => match key.code {
            KeyCode::Char(x) => state.input_text.push(x),
            KeyCode::Esc => {
                state.input_text.clear();
                state.mode = AppMode::Normal;
            }
            KeyCode::Backspace => {
                state.input_text.pop();
            }
            KeyCode::Enter => {
                state.mode = AppMode::Normal;
                config.urls.push(state.input_text.clone());
                state.input_text.clear();
                confy::store("flux_rss", &config).unwrap();
            }
            _ => {}
        },
        AppMode::Help => match key.code {
            KeyCode::Char('h') => state.mode = AppMode::Normal,
            KeyCode::Esc => state.mode = AppMode::Normal,
            _ => {}
        },
        AppMode::Normal => match key.code {
            KeyCode::Char('q') => return false,
            KeyCode::Char('h') => state.mode = AppMode::Help,
            KeyCode::Char('a') => state.mode = AppMode::Input,
            KeyCode::Esc => return false,
            KeyCode::Up => {
                let selection = state.list_state.selected().unwrap().saturating_sub(1);
                state.list_state.select(Some(selection));
            }
            KeyCode::Down => {
                let selection = state.list_state.selected().unwrap().saturating_add(1);
                if selection < config.urls.len() {
                    state.list_state.select(Some(selection));
                }
            }
            _ => {}
        },
    }

    return true;
}
