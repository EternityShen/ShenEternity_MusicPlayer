use crate::data::app::Tab;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use super::super::data::app::App;

pub fn draw(frame: &mut Frame, area: Rect, app: &mut App) {
    let mut spans = Vec::new();

    for tab in Tab::all() {
        if tab == app.tab {
            spans.push(Span::styled(
                format!("[ {} ]", tab.title()),
                Style::default()
                    .fg(ratatui::style::Color::Black)
                    .bg(ratatui::style::Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::styled(
                format!("[ {} ]", tab.title()),
                Style::default(),
            ));
        }
        spans.push(Span::raw(" "));
    }

    let paragraph = Paragraph::new(Line::from(spans)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ratatui::style::Color::LightYellow))
            .title("ShenEternity")
            .title_style(
                Style::default()
                    .fg(ratatui::style::Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
    );

    frame.render_widget(paragraph, area);
}
