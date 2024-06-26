#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{Builder, generate_handler};

#[derive(Serialize, Deserialize)]
struct CharacterSheet {
    name: String,
    class: String,
    level: u8,
    stats: Stats,
    spells: Vec<Spell>,
}

#[derive(Serialize, Deserialize)]
struct Stats {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
}

#[derive(Serialize, Deserialize)]
struct Spell {
    name: String,
    level: u8,
    description: String,
    save_dc: Option<u8>,
}

#[tauri::command]
fn save_character_sheet(character_data: String, file_path: String) -> Result<(), String> {
    let character: CharacterSheet = serde_json::from_str(&character_data)
        .map_err(|e| format!("Failed to parse character data: {}", e))?;
    let toml_data = toml::to_string(&character)
        .map_err(|e| format!("Failed to serialize character data: {}", e))?;

    fs::write(file_path, toml_data)
        .map_err(|e| format!("Failed to write character data to file: {}", e))?;
    Ok(())
}

#[tauri::command]
fn load_character_sheet(file_path: String) -> Result<String, String> {
    let toml_data = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read character data from file: {}", e))?;
    let character: CharacterSheet = toml::from_str(&toml_data)
        .map_err(|e| format!("Failed to parse character data: {}", e))?;
    let character_data = serde_json::to_string(&character)
        .map_err(|e| format!("Failed to serialize character data: {}", e))?;
    Ok(character_data)
}

fn main() {
    Builder::default()
        .invoke_handler(generate_handler![save_character_sheet, load_character_sheet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}