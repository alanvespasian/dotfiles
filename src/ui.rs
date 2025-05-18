use crate::app::{App, AppState, ConnectionStatus};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &mut App) {
    // Main layout with three sections
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1), // Status bar
            Constraint::Min(3),    // Main content area
            Constraint::Length(3), // Instructions
        ])
        .split(f.size());

    // 1. Status Bar
    render_status_bar(f, app, main_chunks[0]);

    // 2. Main Content Area
    render_main_content(f, app, main_chunks[1]);

    // 3. Instructions
    render_instructions(f, app, main_chunks[2]);

    // Popups (render on top of everything)
    if app.show_connection_error {
        render_error_popup(f);
    }
}

fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let status = match app.connection_status {
        ConnectionStatus::Connected => Span::styled(
            "● CONNECTED",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        ConnectionStatus::Disconnected => Span::styled(
            "● DISCONNECTED",
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
        ),
    };

    let now_playing = app.current_station.as_deref().unwrap_or("No station selected");
    let now_playing = Span::styled(
        format!("Now Playing: {}", now_playing),
        Style::default().add_modifier(Modifier::ITALIC),
    );

    let status_bar = Line::from(vec![status, Span::raw(" | "), now_playing]);
    f.render_widget(Paragraph::new(status_bar), area);
}

fn render_main_content(f: &mut Frame, app: &mut App, area: Rect) {
    match app.state {
        AppState::CountrySelection => render_countries_view(f, app, area),
        AppState::RadioSelection => render_stations_view(f, app, area),
    }
}

fn render_countries_view(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left panel: Countries list
    let countries: Vec<ListItem> = app.countries
        .iter()
        .map(|country| {
            ListItem::new(Line::from(Span::styled(
                country.name.clone(),
                Style::default(),
            )))
        })
        .collect();

    let list = List::new(countries)
        .block(
            Block::default()
                .title(" Countries ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, chunks[0], &mut app.list_state);

    // Right panel: Country info
    let info = if let Some(selected) = app.list_state.selected() {
        if let Some(country) = app.countries.get(selected) {
            let station_list = country.stations.iter()
                .take(5) // Show first 5 stations as preview
                .map(|s| s.name.as_str())
                .collect::<Vec<_>>()
                .join("\n");
            
            format!(
                "Continent: {}\n\nStations: {}\n\n{}",
                country.continent,
                country.stations.len(),
                station_list
            )
        } else {
            "Select a country".to_string()
        }
    } else {
        "Select a country".to_string()
    };

    let info_block = Block::default()
        .title(" Country Info ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue));

    f.render_widget(
        Paragraph::new(info).block(info_block),
        chunks[1],
    );
}

fn render_stations_view(f: &mut Frame, app: &mut App, area: Rect) {
    let stations: Vec<ListItem> = app.current_stations()
        .iter()
        .map(|station| {
            let style = if Some(&station.name) == app.current_station.as_ref() {
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            ListItem::new(Line::from(Span::styled(
                station.name.clone(),
                style,
            )))
        })
        .collect();

    let list = List::new(stations)
        .block(
            Block::default()
                .title(" Radio Stations ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, area, &mut app.list_state);
}

fn render_instructions(f: &mut Frame, app: &App, area: Rect) {
    let instructions = match app.state {
        AppState::CountrySelection => vec![
            Line::from("↑/↓: Navigate"),
            Line::from("Enter: Select country"),
            Line::from("q: Quit"),
        ],
        AppState::RadioSelection => vec![
            Line::from("↑/↓: Navigate"),
            Line::from("Enter: Play station"),
            Line::from("s: Stop playback"),
            Line::from("Esc: Back to countries"),
            Line::from("q: Quit"),
        ],
    };

    let block = Block::default()
        .title(" Controls ")
        .borders(Borders::TOP)
        .border_style(Style::default().fg(Color::DarkGray));

    f.render_widget(
        Paragraph::new(instructions).block(block),
        area,
    );
}

fn render_error_popup(f: &mut Frame) {
    let block = Block::default()
        .title(" ERROR ")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Red).fg(Color::White));

    let text = vec![
        Line::from(Span::styled(
            "NO INTERNET CONNECTION",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Streaming requires an active internet connection"),
        Line::from(""),
        Line::from("Press any key to continue"),
    ];

    let area = centered_rect(60, 25, f.size());
    f.render_widget(Clear, area);
    f.render_widget(Paragraph::new(text).block(block), area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
