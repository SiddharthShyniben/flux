use tui::{
    layout::{Constraint::Percentage, Direction::Horizontal, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{List, ListItem, Paragraph, Wrap},
};

use crate::{
    models::AppState,
    utils::{centered_rect, make_box},
};

pub fn default_layout(size: Rect) -> std::rc::Rc<[Rect]> {
    Layout::default()
        .direction(Horizontal)
        .constraints([Percentage(50), Percentage(50)])
        .split(size)
}

pub fn make_url_list(urls: &Vec<String>) -> List {
    let list_items: Vec<ListItem<'_>> = urls
        .iter()
        .map(|url| ListItem::new(url.to_string()))
        .collect();

    List::new(list_items)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(" >> ")
        .block(make_box(Some("Flux")))
}

pub fn make_nocontent_page() -> Paragraph<'static> {
    Paragraph::new(Span::styled(
        " Your feed is empty.".to_string(),
        Style::default().add_modifier(Modifier::DIM),
    ))
    .block(make_box(Some("Flux")))
}

const HELP: &str = " q - Quit\n h - Toggle help menu\n a - Add URL to feed";

pub fn make_help_box() -> Paragraph<'static> {
    Paragraph::new(HELP)
        .block(make_box(Some("Help")))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false })
}

pub fn make_input<'a>(size: Rect, state: &'a AppState) -> (Rect, Paragraph<'a>) {
    let area = centered_rect(50, 10, size);
    let input_area = Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: 3,
    };

    let input = Paragraph::new(state.input_text.as_ref())
        .block(make_box(Some("RSS URL")))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });

    (input_area, input)
}
