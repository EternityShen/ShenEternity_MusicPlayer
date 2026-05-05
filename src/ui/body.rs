use std::{borrow::Borrow, time::Duration};

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
};

use crate::data::app::App;

pub fn draw(frame: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
        .spacing(1)
        .split(area);

    let left_area = chunks[0];
    let right_area = chunks[1];

    let left_chunks = Layout::vertical([
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(40),
    ])
    .split(left_area);

    let left_top_area = left_chunks[0];
    let left_mid_area = left_chunks[1];
    let left_bottom_area = left_chunks[2];

    let songs: Vec<String> = app.songs.iter().map(|song| song.title.clone()).collect();

    app.list_state
        .select(Some(app.list_state.selected().unwrap_or(0)));

    let items: Vec<ListItem> = songs
        .iter()
        .enumerate()
        .map(|(i, song)| {
            let is_selected = app.list_state.selected() == Some(i);

            let style =
                if app.playing_song.is_some() && is_selected && i == app.playing_song.unwrap() {
                    Style::default().fg(ratatui::style::Color::LightMagenta)
                } else if app.playing_song.is_some()
                    && *song == app.songs[app.playing_song.unwrap()].title
                {
                    Style::default().fg(ratatui::style::Color::LightYellow)
                } else if is_selected {
                    Style::default().fg(ratatui::style::Color::LightBlue)
                } else {
                    Style::default()
                };

            let span =
                if app.playing_song.is_some() && is_selected && i == app.playing_song.unwrap() {
                    Span::styled(
                        format!(
                            "> >> >>>>{}<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<",
                            song.clone()
                        ),
                        style,
                    )
                } else if app.playing_song.is_some()
                    && *song == app.songs[app.playing_song.unwrap()].title
                {
                    Span::styled(
                        format!(
                            ">> >>>>>>{}<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<",
                            song.clone()
                        ),
                        style,
                    )
                } else if is_selected {
                    Span::styled(
                        format!(
                            "> >>>>>>{}<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<",
                            song.clone()
                        ),
                        style,
                    )
                } else {
                    Span::raw(format!(
                        "  >>>>>>>{}<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<",
                        song.clone()
                    ))
                };

            ListItem::new(Line::from(span))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ratatui::style::Color::Red))
            .title("音乐列表")
            .title_style(Style::default().fg(ratatui::style::Color::LightRed)),
    );

    frame.render_stateful_widget(list, left_top_area, &mut app.list_state);

    let left_mid = Paragraph::new("播放模式: 列表循环")
        .style(Style::default().fg(ratatui::style::Color::LightYellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(ratatui::style::Color::LightYellow))
                .title("播放模式")
                .title_style(Style::default().fg(ratatui::style::Color::Yellow)),
        );

    frame.render_widget(left_mid, left_mid_area);

    let left_bottom_paragraph = if let Some(playing) = app.playing_song {
        Paragraph::new(format!(
            "播放中: [{}]\n时长: [{}]\n当前播放到: [{}%]",
            app.songs[playing].title,
            app.player
                .duration
                .unwrap_or(Duration::from_secs(0))
                .as_secs_f64(),
            (app.get_pragress() * 100.0) as u64
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(ratatui::style::Color::LightYellow))
                .title("正在播放")
                .title_style(
                    Style::default()
                        .fg(ratatui::style::Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
        )
    } else {
        Paragraph::new("没有音乐在播放").block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(ratatui::style::Color::LightYellow))
                .title("正在播放")
                .title_style(
                    Style::default()
                        .fg(ratatui::style::Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
        )
    };

    frame.render_widget(left_bottom_paragraph, left_bottom_area);

    let right_chunks =
        Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(right_area);

    let gauge = Gauge::default()
        .block(
            Block::default()
                .title("进度")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(ratatui::style::Color::Blue))
                .title_style(
                    Style::default()
                        .fg(ratatui::style::Color::Blue)
                        .add_modifier(Modifier::BOLD),
                ),
        )
        .gauge_style(Style::default().fg(ratatui::style::Color::LightBlue))
        .ratio(app.get_pragress())
        .label("");
    frame.render_widget(gauge, right_chunks[0]);

    let right_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ratatui::style::Color::LightMagenta))
        .title("未知")
        .title_style(
            Style::default()
                .fg(ratatui::style::Color::Magenta)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(right_block, right_chunks[1]);
}
