use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, widgets::Clear, Terminal};

use std::io;
use unicode_width::UnicodeWidthStr;

mod models;
mod ui;
mod utils;
mod keyboard;

use models::{AppMode, AppState, Config};
use ui::{default_layout, make_help_box, make_input, make_nocontent_page, make_url_list};
use utils::{centered_rect, make_box};
use keyboard::handle_event;

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
            let chunks = default_layout(size);

            if config.urls.len() > 0 {
                let list = make_url_list(&config.urls);
                f.render_stateful_widget(list, chunks[0], &mut state.list_state);
            } else {
                f.render_widget(make_nocontent_page(), chunks[0]);
            }

            let view = make_box(None);
            f.render_widget(view, chunks[1]);

            if let AppMode::Help = state.mode {
                let area = centered_rect(50, 30, size);
                let help = make_help_box();
                f.render_widget(Clear, area);
                f.render_widget(help, area);
            }

            if let AppMode::Input = state.mode {
                let (input_area, input) = make_input(size, &state);

                f.set_cursor(
                    input_area.x + state.input_text.width() as u16 + 1,
                    input_area.y + 1,
                );

                f.render_widget(Clear, input_area);
                f.render_widget(input, input_area);
            }
        })?;

        if let Event::Key(key) = event::read()? {
            if !handle_event(&mut config, &mut state, key) {break}
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
