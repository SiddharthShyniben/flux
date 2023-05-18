use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};

use serde_derive::{Deserialize, Serialize};

use std::{default, io};
use unicode_width::UnicodeWidthStr;

#[derive(Default, Debug, Serialize, Deserialize)]
struct Config {
    urls: Vec<String>,
}

#[derive(Default)]
enum AppMode {
    #[default]
    Normal,
    Help,
    Input,
}

struct AppState {
    pub mode: AppMode,
    pub help: bool,
    pub input: bool,
    pub input_text: String,
    pub list_state: ListState,
}

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
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(size);

            let list = Block::default().title(" Flux ").borders(Borders::ALL);
            if config.urls.len() > 0 {
                let list_items: Vec<ListItem<'_>> = config
                    .urls
                    .iter()
                    .map(|url| ListItem::new(url.to_string()))
                    .collect();

                let list = List::new(list_items)
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                    .highlight_symbol(" >> ")
                    .block(list);
                f.render_stateful_widget(list, chunks[0], &mut state);
            } else {
                let list = Paragraph::new(Span::styled(
                    " Your feed is empty.",
                    Style::default().add_modifier(Modifier::DIM),
                ))
                .block(list);
                f.render_widget(list, chunks[0]);
            };
            let view = Block::default().borders(Borders::ALL);
            f.render_widget(view, chunks[1]);

            if let AppMode::Help = state.mode {
                let block = Block::default().title(" Help ").borders(Borders::ALL);
                let area = centered_rect(50, 30, size);
                let text = vec!["q - Quit", "h - Toggle help menu"]
                    .iter()
                    .map(|&s| " ".to_owned() + s)
                    .collect::<Vec<_>>()
                    .join("\n");
                let help_text = Paragraph::new(text)
                    .block(block)
                    .style(Style::default().fg(Color::White))
                    .wrap(Wrap { trim: false });
                f.render_widget(Clear, area);
                f.render_widget(help_text, area);
            }

            if AppMode::Input = state.mode {
                let block = Block::default().title(" RSS URL ").borders(Borders::ALL);
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
                    // Put cursor past the end of the input text
                    input_area.x + state.input_text.width() as u16 + 1,
                    // Move one line down, from the border to the input line
                    input_area.y + 1,
                );
                f.render_widget(Clear, input_area);
                f.render_widget(input, input_area);
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match state.mode {
                AppMode::Input => match key.code {
                    KeyCode::Char(x) => {
                        state.input_text.push(x);
                    }
                    KeyCode::Esc => {
                        state.input_text.drain(..);
                        state.mode = AppMode::Normal;
                    }
                    KeyCode::Backspace => {
                        state.input_text.pop();
                    }
                    KeyCode::Enter => {
                        state.mode = AppMode::Normal;
                        config.urls.push(state.input_text.clone());
                        state.input_text.drain(..);
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
                    KeyCode::Char('a') => add_url = true,
                    KeyCode::Esc => {
                        if help {
                            help = false
                        } else {
                            break;
                        }
                    }
                    KeyCode::Up => {
                        let selection = state.selected().unwrap().saturating_sub(1);
                        state.select(Some(selection));
                    }
                    KeyCode::Down => {
                        let selection = state.selected().unwrap().saturating_add(1);
                        if selection < config.urls.len() {
                            state.select(Some(selection));
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

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
