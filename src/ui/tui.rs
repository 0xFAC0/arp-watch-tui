use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::error;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame, Terminal,
};
use std::{error::Error, io, time::Duration};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerWidget};

use crate::net_arp::NetArpSenderMutex;

use super::{arp_cache_widget::ArpCacheWidget, helper::helper, App, UiSettings};

pub async fn main_tui(app: App) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    run_app(&mut term, app).await?;

    term.clear()?;
    disable_raw_mode().unwrap();
    execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;
    Ok(())
}

pub async fn run_app<B: Backend>(
    term: &mut Terminal<B>,
    mut app: App,
) -> Result<(), Box<dyn Error>> {
    loop {
        let ui_settings = app.get_ui_settings().await;
        term.draw(|f| draw(f, ui_settings))?;

        if poll(Duration::from_millis(100)).unwrap() {
            // Will not block thanks to event::poll
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('s') => {
                        let sender_mutex: NetArpSenderMutex = app.net_sender.clone();
                        tokio::spawn(async move {
                            let mut sender = sender_mutex.lock().await;
                            if let Err(e) = sender.scan_network().await {
                                error!("Scan hosts failed {e}");
                            }
                        });
                    }
                    KeyCode::Char('f') => {
                        app.toggle_follow_mode().await;
                    }
                    _ => continue,
                };
            }
        }
    }
    Ok(())
}

pub fn draw<B: Backend>(frame: &mut Frame<B>, ui_settings: UiSettings) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray))
        .border_type(BorderType::Rounded);

    let tui_log = TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Magenta))
        .style_info(Style::default().fg(Color::Cyan))
        .output_separator(':')
        .output_timestamp(Some("%H:%M:%S".to_string()))
        .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
        .output_target(true)
        .output_file(true)
        .output_line(true)
        .block(
            block
                .clone()
                .title("Logs")
                .title_alignment(Alignment::Center),
        );

    let arp_cache_widget = ArpCacheWidget::default()
        .block(
            block
                .clone()
                .title("ARP Cache")
                .title_alignment(Alignment::Center),
        )
        .entries(&ui_settings.arp_entries);

    let helper = helper(&ui_settings).block(block.clone());

    let root_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Min(0), Constraint::Max(3)].as_ref())
        .split(frame.size());

    let body_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)].as_ref())
        .split(root_layout[0]);

    frame.render_widget(tui_log, body_layout[0]);
    frame.render_widget(arp_cache_widget, body_layout[1]);
    frame.render_widget(helper, root_layout[1]);
}
