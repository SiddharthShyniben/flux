use tui::{layout::{
    Constraint::Percentage,
    Direction::Horizontal,
    Layout, Rect
}, widgets::{ListItem, List, Paragraph}, style::{Style, Modifier}, text::Span};

use crate::utils::make_box;

pub fn default_layout(size: Rect) -> std::rc::Rc<[Rect]> {
    Layout::default()
        .direction(Horizontal)
        .constraints([Percentage(50), Percentage(50)])
        .split(size)
}

pub fn make_url_list(urls: &Vec<String>) -> List {
    let list_items: Vec<ListItem<'_>> = urls.iter()
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
    )).block(make_box(Some("Flux")))
}
