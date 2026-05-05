use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::data::app::App;

pub mod body;
pub mod footer;
pub mod header;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .split(area);

    let header_area = chunks[0];
    let body_area = chunks[1];
    let footer_area = chunks[2];
    header::draw(frame, header_area, app);
    body::draw(frame, body_area, app);
    footer::draw(frame, footer_area);
}
