use crossterm::{
    event::{self, EnableMouseCapture, Event, KeyCode, DisableMouseCapture},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Block, Borders},
    Frame, Terminal, layout::{Layout, Constraint},
};

use crate::net_arp::NetArpSenderMutex;

use super::App;

pub async fn main_tui(app: App) {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend).unwrap();

    run_app(&mut term, app).await;

    term.clear().unwrap();
    disable_raw_mode().unwrap();
    execute!(term.backend_mut(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
    term.show_cursor().unwrap();
}

pub async fn run_app<B: Backend>(term: &mut Terminal<B>, app: App) {
    loop {
        term.draw(draw).unwrap();

        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('s') => {
                    let sender_mutex: NetArpSenderMutex = app.net_sender.clone();
                    tokio::spawn(async move {
                        let mut sender = sender_mutex.lock().await;
                        sender.scan_network().await;
                    });
                },
                _ => continue
            };
        }
    }
}

pub fn draw<B: Backend>(frame: &mut Frame<B>) {
    let main_b = Block::default().borders(Borders::ALL).title("Box");
    frame.render_widget(main_b, frame.size());
}
