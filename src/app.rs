use crate::models::{Country, Station};
use crossterm::event::KeyCode;
use ratatui::widgets::ListState;
use reqwest::blocking::Client;
use std::time::Duration;
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    CountrySelection,
    RadioSelection,
}

pub struct App {
    pub countries: Vec<Country>,
    pub state: AppState,
    pub list_state: ListState,
    pub connection_status: ConnectionStatus,
    pub show_connection_error: bool,
    pub current_station: Option<String>,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            countries: crate::radio::load_radio_data()?,
            state: AppState::CountrySelection,
            list_state: ListState::default().with_selected(Some(0)),
            connection_status: ConnectionStatus::Disconnected,
            show_connection_error: false,
            current_station: None,
        })
    }

    pub fn initialize(&mut self) {
        self.check_connection_status();
    }

    fn check_connection(&self) -> ConnectionStatus {
        match Client::new()
            .get("http://www.google.com")
            .timeout(Duration::from_secs(3))
            .send()
        {
            Ok(_) => ConnectionStatus::Connected,
            Err(_) => ConnectionStatus::Disconnected,
        }
    }

    pub fn check_connection_status(&mut self) {
        self.connection_status = self.check_connection();
        self.show_connection_error = self.connection_status == ConnectionStatus::Disconnected;
    }

    pub fn find_station(&self, name: &str) -> Option<&Station> {
        self.countries
            .iter()
            .flat_map(|country| &country.stations)
            .find(|station| station.name == name)
    }

    pub fn next(&mut self) {
        let len = self.current_items_len();
        if len == 0 {
            return;
        }
        
        let new_selection = match self.list_state.selected() {
            Some(current) if current >= len - 1 => 0,
            Some(current) => current + 1,
            None => 0,
        };
        
        self.list_state.select(Some(new_selection));
    }

    pub fn previous(&mut self) {
        let len = self.current_items_len();
        if len == 0 {
            return;
        }
        
        let new_selection = match self.list_state.selected() {
            Some(0) => len - 1,
            Some(current) => current - 1,
            None => len.saturating_sub(1),
        };
        
        self.list_state.select(Some(new_selection));
    }

    fn current_items_len(&self) -> usize {
        match self.state {
            AppState::CountrySelection => self.countries.len(),
            AppState::RadioSelection => self.current_country()
                .map_or(0, |c| c.stations.len()),
        }
    }

    pub fn current_country(&self) -> Option<&Country> {
        if let AppState::CountrySelection = self.state {
            return None;
        }
        
        self.list_state.selected()
            .and_then(|i| self.countries.get(i))
    }

    pub fn current_stations(&self) -> Vec<&Station> {
        self.current_country()
            .map(|country| country.stations.iter().collect())
            .unwrap_or_default()
    }

    pub fn handle_key(&mut self, key: KeyCode) -> Option<String> {
        match key {
            KeyCode::Down => {
                self.next();
                None
            }
            KeyCode::Up => {
                self.previous();
                None
            }
            KeyCode::Enter => self.select(),
            KeyCode::Esc => {
                self.back();
                None
            }
            _ => None,
        }
    }

    pub fn select(&mut self) -> Option<String> {
        match self.state {
            AppState::CountrySelection => {
                if self.countries.is_empty() {
                    return None;
                }
                
                self.state = AppState::RadioSelection;
                self.list_state.select(Some(0));
                None
            }
            AppState::RadioSelection => {
                let station_name = self.current_stations()
                    .get(self.list_state.selected()?)
                    .map(|s| s.name.clone());
                
                self.current_station = station_name.clone();
                station_name
            }
        }
    }

    pub fn back(&mut self) {
        self.state = AppState::CountrySelection;
        self.current_station = None;
        
        if !self.countries.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    pub fn currently_playing(&self) -> Option<&str> {
        self.current_station.as_deref()
    }
}
