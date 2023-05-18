use tui::{
    layout::{
        Constraint::Percentage,
        Direction::{Horizontal, Vertical},
        Layout, Rect,
    },
    widgets::{Block, Borders},
};

pub fn make_box(title: Option<&str>) -> Block {
    match title {
        Some(title) => Block::default()
            .title(format!(" {} ", title))
            .borders(Borders::ALL),
        None => Block::default().borders(Borders::ALL),
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Vertical)
        .constraints(
            [
                Percentage((100 - percent_y) / 2),
                Percentage(percent_y),
                Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Horizontal)
        .constraints(
            [
                Percentage((100 - percent_x) / 2),
                Percentage(percent_x),
                Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
