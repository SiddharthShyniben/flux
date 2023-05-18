use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint::Percentage, Direction::Horizontal, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Clear, List, ListItem, Paragraph, Wrap},
    Terminal,
};
use ui::{make_url_list, make_nocontent_page};

use std::io;
use unicode_width::UnicodeWidthStr;

mod models;
mod utils;
mod ui;
use models::{AppMode, AppState, Config};
use utils::{centered_rect, make_box};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut config: Config = confy::load("flux_rss")?;
    let mut state = AppState::default();
    state.list_state.select(Some(0));

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = ui::default_layout(size);

            if config.urls.len() > 0 {
                let list = make_url_list(&config.urls);
                f.render_stateful_widget(list, chunks[0], &mut state.list_state);
            } else {
                let no_content = make_nocontent_page();
                f.render_widget(no_content, chunks[0]);
            }

            let view = make_box(None);
            f.render_widget(view, chunks[1]);

            if let AppMode::Help = state.mode {
                let block = make_box(Some("Help"));
                let area = centered_rect(50, 30, size);
                let text = vec![" q - Quit", " h - Toggle help menu"].join("\n");
                let help_text = Paragraph::new(text)
                    .block(block)
                    .style(Style::default().fg(Color::White))
                    .wrap(Wrap { trim: false });

                f.render_widget(Clear, area);
                f.render_widget(help_text, area);
            }

            if let AppMode::Input = state.mode {
                let block = make_box(Some("RSS URL"));
                let area = centered_rect(50, 10, size);
                let input_area = Rect {
                    x: area.x,
                    y: area.y,
                    width: area.width,
                    height: 3,
                };
                let input = Paragraph::new(state.input_text.as_ref())
                    .block(block)
                    .style(Style::default().fg(Color::White))
                    .wrap(Wrap { trim: true });

                f.set_cursor(
                    input_area.x + state.input_text.width() as u16 + 1,
                    input_area.y + 1,
                );

                f.render_widget(Clear, input_area);
                f.render_widget(input, input_area);
            }
        })?;

        if let Event::Key(key) = event::read()? {
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
                        confy::store("flux_rss", &config)?;
                    }
                    _ => {}
                },
                AppMode::Help => match key.code {
                    KeyCode::Char('h') => state.mode = AppMode::Normal,
                    KeyCode::Esc => state.mode = AppMode::Normal,
                    _ => {}
                },
                AppMode::Normal => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('h') => state.mode = AppMode::Help,
                    KeyCode::Char('a') => state.mode = AppMode::Input,
                    KeyCode::Esc => break,
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
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
