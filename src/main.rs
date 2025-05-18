mod app;
mod models;
mod radio;
mod ui;

use anyhow::{Context, Result};
use app::{App, ConnectionStatus};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io,
    process,
    time::{Duration, Instant},
};

fn main() -> Result<()> {
    // Initialize terminal
    let mut terminal = setup_terminal()?;

    // Create and initialize application state
    let mut app = App::new()?;
    app.initialize();

    // Run main application loop
    let result = run_app(&mut terminal, &mut app);

    // Cleanup terminal
    restore_terminal(&mut terminal)?;

    result
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture
    )?;
    Terminal::new(CrosstermBackend::new(stdout)).context("Failed to create terminal")
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    let mut last_connection_check = Instant::now();
    let connection_check_interval = Duration::from_secs(5);
    let mut player_process = None;

    loop {
        terminal.draw(|f| ui::render(f, app))?;

        // Check connection periodically
        if last_connection_check.elapsed() > connection_check_interval {
            app.check_connection_status();
            last_connection_check = Instant::now();
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if app.show_connection_error {
                    app.show_connection_error = false;
                    continue;
                }

                match key.code {
                    KeyCode::Char('q') => {
                        stop_player(&mut player_process);
                        return Ok(());
                    }
                    KeyCode::Char('s') => {
                        stop_player(&mut player_process);
                    }
                    KeyCode::Esc => {
                        app.back();
                    }
                    _ => {
                        if let Some(station_name) = app.handle_key(key.code) {
                            if let ConnectionStatus::Connected = app.connection_status {
                                if let Some(station) = app.find_station(&station_name) {
                                    stop_player(&mut player_process);
                                    player_process = start_player(&station.url)?;
                                    println!("Now playing: {}", station.name);
                                }
                            } else {
                                app.show_connection_error = true;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn start_player(url: &str) -> Result<Option<process::Child>> {
    Ok(process::Command::new("cvlc")
        .arg("--no-video")
        .arg("--play-and-exit")
        .arg(url)
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .spawn()
        .ok())
}

fn stop_player(player: &mut Option<process::Child>) {
    if let Some(mut child) = player.take() {
        let _ = child.kill();
    }
}
