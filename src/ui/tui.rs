use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::error;
use std::{io, error::Error};
use tui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame, Terminal,
};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerWidget};

use crate::net_arp::NetArpSenderMutex;

use super::App;

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

pub async fn run_app<B: Backend>(term: &mut Terminal<B>, app: App) -> Result<(), Box<dyn Error>> {
    loop {
        term.draw(draw)?;

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
                _ => continue,
            };
        }
    }
    Ok(())
}

pub fn draw<B: Backend>(frame: &mut Frame<B>) {
    let tui_sm = TuiLoggerWidget::default()
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
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default())
                .title("Logs")
        );

    frame.render_widget(tui_sm, frame.size());
}
