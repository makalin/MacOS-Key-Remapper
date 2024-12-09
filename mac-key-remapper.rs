use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use enigo::{Enigo, KeyboardControllable};
use serde::{Deserialize, Serialize};

const CONFIG_FILE_NAME: &str = ".mac-key-remapper.json";

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    browser_url: String,
    text_shortcuts: Vec<TextShortcut>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TextShortcut {
    trigger: char,
    text: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            browser_url: String::from("https://www.google.com"),
            text_shortcuts: vec![
                TextShortcut {
                    trigger: 'h',
                    text: String::from("Hello, World!"),
                },
                TextShortcut {
                    trigger: 't',
                    text: String::from("Thank you very much"),
                },
            ],
        }
    }
}

struct KeyHandler {
    config: Config,
    enigo: Enigo,
}

impl KeyHandler {
    fn new() -> Self {
        let config = load_config().unwrap_or_default();
        Self {
            config,
            enigo: Enigo::new(),
        }
    }

    fn handle_caps_lock(&mut self) {
        if let Err(e) = open::that(&self.config.browser_url) {
            eprintln!("Failed to open URL: {}", e);
        }
    }

    fn handle_fn_key(&mut self, c: char) {
        if let Some(shortcut) = self.config.text_shortcuts.iter().find(|s| s.trigger == c) {
            self.enigo.key_sequence(&shortcut.text);
        }
    }
}

fn load_config() -> Result<Config, Box<dyn Error>> {
    let config_path = dirs::home_dir()
        .ok_or("Could not find home directory")?
        .join(CONFIG_FILE_NAME);

    if !config_path.exists() {
        let config = Config::default();
        let mut file = File::create(&config_path)?;
        serde_json::to_writer_pretty(&mut file, &config)?;
        return Ok(config);
    }

    let file = File::open(config_path)?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)?;
    Ok(config)
}

fn create_key_monitor_script() -> Result<PathBuf, Box<dyn Error>> {
    let script_content = r#"#!/usr/bin/osascript
tell application "System Events"
    repeat
        set caps_state to get capslock of keyboard
        if caps_state is true then
            do shell script "echo 'CAPS_LOCK' >> /tmp/key_events"
            delay 0.5
        end if
        
        set fn_state to get key code 63
        if fn_state is down then
            set input to text returned of (display dialog "Enter trigger character:" default answer "")
            do shell script "echo 'FN:" & input & "' >> /tmp/key_events"
        end if
    end repeat
end tell
"#;

    let script_path = dirs::home_dir()
        .ok_or("Could not find home directory")?
        .join("key_monitor.scpt");
    
    let mut file = File::create(&script_path)?;
    file.write_all(script_content.as_bytes())?;
    
    Ok(script_path)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting Key Remapper...");
    
    let script_path = create_key_monitor_script()?;
    
    // Start the AppleScript
    std::process::Command::new("osascript")
        .arg(&script_path)
        .spawn()?;

    // Set up Ctrl+C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    let mut key_handler = KeyHandler::new();
    
    // Monitor /tmp/key_events for key events
    while running.load(Ordering::SeqCst) {
        if let Ok(content) = std::fs::read_to_string("/tmp/key_events") {
            if !content.is_empty() {
                for line in content.lines() {
                    match line {
                        "CAPS_LOCK" => key_handler.handle_caps_lock(),
                        l if l.starts_with("FN:") => {
                            if let Some(c) = l.chars().nth(3) {
                                key_handler.handle_fn_key(c);
                            }
                        }
                        _ => {}
                    }
                }
                // Clear the file
                std::fs::write("/tmp/key_events", "")?;
            }
        }
        thread::sleep(Duration::from_millis(100));
    }

    // Cleanup
    std::fs::remove_file(script_path)?;
    std::fs::remove_file("/tmp/key_events")?;
    
    println!("Stopped.");
    Ok(())
}
