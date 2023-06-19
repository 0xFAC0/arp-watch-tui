use super::*;
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Line},
    widgets::Paragraph,
};

pub fn helper<'a>(ui_settings: &UiSettings) -> Paragraph<'a> {
    let enabled_color = |enabled| {
        match enabled {
            true => Color::Green,
            false => Color::Red,
        }
    };
    let follow_mode_color = enabled_color(ui_settings.follow_mode);
    let rearp_color = enabled_color(ui_settings.rearp_enable);

    let line: Line = vec![
        Span::raw(" | [s] scan for hosts | "),
        Span::raw("[f] "),
        Span::styled("⬤", Style::default().fg(follow_mode_color)),
        Span::raw(" Toggle follow mode | "),
        Span::raw("[r] "),
        Span::styled("⬤", Style::default().fg(rearp_color)),
        Span::raw(" Toggle reARPing mode | "),
    ].into();

    Paragraph::new(line)
    .alignment(Alignment::Center)
}
