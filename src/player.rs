// src/player.rs
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct RadioPlayer {
    process: Arc<Mutex<Option<std::process::Child>>>,
}

impl RadioPlayer {
    pub fn new() -> Self {
        RadioPlayer {
            process: Arc::new(Mutex::new(None)),
        }
    }

    pub fn play(&self, url: &str) {
        self.stop(); // Stop any currently playing stream

        let url = url.to_string();
        let process = Arc::clone(&self.process);
        thread::spawn(move || {
            let child = Command::new("cvlc")
                .arg("--no-video")
                .arg("--play-and-exit")
                .arg(url)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn();

            if let Ok(child) = child {
                *process.lock().unwrap() = Some(child);
            }
        });
    }

    pub fn stop(&self) {
        if let Some(mut child) = self.process.lock().unwrap().take() {
            let _ = child.kill();
        }
    }
}
