use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(frame: &mut Frame, area: Rect) {
    let paragraph = Paragraph::new("切换TAb: j, 上一首: k, 下一首: l, 播放/暂停/继续: p").block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ratatui::style::Color::LightYellow))
            .title("按键")
            .title_style(
                Style::default()
                    .fg(ratatui::style::Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
    );
    frame.render_widget(paragraph, area);
}
