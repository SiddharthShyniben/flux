use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde_derive::{Deserialize, Serialize};
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Clear, Paragraph, Wrap, List, ListItem},
    Terminal, text::Span,
};

#[derive(Default, Debug, Serialize, Deserialize)]
struct Config {
    urls: Vec<String>,
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut help = false;
    let mut add_url = true;
    let mut text = "https://";
    let config: Config = confy::load("flux_rss")?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(size);

            let list = Block::default().title(" Flux ").borders(Borders::ALL);
            if config.urls.len() > 0 {
                let list_items: Vec<ListItem<'_>> = config.urls
                    .iter()
                    .map(|url| ListItem::new(url.to_string()))
                    .collect();

                let list = List::new(list_items);
                f.render_widget(list, chunks[0]);
            } else {
                let list = Paragraph::new(Span::styled(" Your feed is empty.", Style::default().add_modifier(Modifier::DIM)))
                    .block(list);
                f.render_widget(list, chunks[0]);
            };
            let view = Block::default().borders(Borders::ALL);
            f.render_widget(view, chunks[1]);

            if help {
                let block = Block::default().title(" Help ").borders(Borders::ALL);
                let area = centered_rect(50, 30, size);
                let text = vec!["q - Quit", "h - Toggle help menu"];
                let text = text
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
            
            if add_url {
                let block = Block::default().title(" Help ").borders(Borders::ALL);
                let area = centered_rect(50, 10, size);
                let input_area = Rect {
                    x: area.x, y: area.y, width: area.width, height: 3
                };
                let input = Paragraph::new(text)
                    .block(block)
                    .style(Style::default().fg(Color::White))
                    .wrap(Wrap { trim: true });
                f.render_widget(Clear, input_area);
                f.render_widget(input, input_area);
            }
        })?;

        if let Event::Key(key) = event::read()? {
            if add_url {
                match key.code {
                    KeyCode::Char(x) => text += x,
                    KeyCode::Esc => {
                        text = "https://";
                        add_url = false
                    }
                }
            }
            else {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('h') => help = !help,
                    KeyCode::Char('a') => add_url = true,
                    KeyCode::Esc => {
                        if help {
                            help = false
                        } else {
                            break;
                        }
                    }
                    _ => {}
                }
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
