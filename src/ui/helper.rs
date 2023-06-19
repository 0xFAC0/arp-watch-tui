use super::*;
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Line},
    widgets::Paragraph,
};

pub fn helper<'a>(ui_settings: &UiSettings) -> Paragraph<'a> {
    let follow_mode_color = match ui_settings.follow_mode {
        true => Color::Green,
        false => Color::Red,
    };

    let line: Line = vec![
        Span::raw(" | [s] scan for hosts | "),
        Span::raw("[f] "),
        Span::styled("⬤", Style::default().fg(follow_mode_color)),
        Span::raw(" Toggle follow mode | "),
    ].into();

    Paragraph::new(line)
    .alignment(Alignment::Center)
}
