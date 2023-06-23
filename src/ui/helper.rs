use super::*;
use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

pub fn button<'a>(
    letter: char,
    helper_msg: &'static str,
    activated: Option<bool>,
) -> Vec<Span<'a>> {
    let color = match activated {
        Some(activated) => match activated {
            true => Color::Green,
            false => Color::Red,
        },
        None => Color::Cyan,
    };
    vec![
        Span::styled(" ", Style::default().fg(color)),
        Span::styled(
            format!("{}", letter),
            Style::default()
                .bg(color)
                .add_modifier(Modifier::UNDERLINED),
        ),
        Span::styled(format!(" {}", helper_msg), Style::default().bg(color)),
        Span::styled(" ", Style::default().fg(color)),
    ]
}

pub fn helper<'a>(ui_settings: &UiSettings) -> Paragraph<'a> {
    let mut line: Line = Line::from(vec![]);
    let mut help_scan = button('S', "Scan hosts", None);
    let mut toggle_follow = button('F', "Toggle follow mode", Some(ui_settings.follow_mode));
    line.spans.append(&mut help_scan);
    line.spans.append(&mut toggle_follow);
    Paragraph::new(line).alignment(Alignment::Center)
}
