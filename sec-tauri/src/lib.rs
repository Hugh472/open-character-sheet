use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use toml;
use tauri::api::dialog::blocking::FileDialogBuilder;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub name: String,
    pub class: String,
    pub level: u8,
    pub race: String,
    pub background: String,
    pub alignment: String,
    pub experience_points: u32,
    pub ability_scores: AbilityScores,
    pub skill_proficiencies: Vec<String>,
    pub saving_throw_proficiencies: Vec<String>,
    pub spell_slots: SpellSlots,
    pub inventory: Vec<Item>,
    pub spells: Vec<Spell>,
    pub class_features: Vec<String>,
    pub spell_attack_bonus: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AbilityScores {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpellSlots {
    pub level_1: u8,
    pub level_2: u8,
    pub level_3: u8,
    pub level_4: u8,
    pub level_5: u8,
    pub level_6: u8,
    pub level_7: u8,
    pub level_8: u8,
    pub level_9: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub name: String,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spell {
    pub name: String,
    pub level: u8,
    pub description: String,
    pub save_dc: Option<u8>,
}

#[tauri::command]
pub fn load_characters(directory: String) -> Result<Vec<Character>, String> {
    let mut characters = Vec::new();
    let paths = fs::read_dir(directory).map_err(|e| e.to_string())?;

    for path in paths {
        let path = path.map_err(|e| e.to_string())?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let contents = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            let character: Character = toml::from_str(&contents).map_err(|e| e.to_string())?;
            characters.push(character);
        }
    }

    Ok(characters)
}

#[tauri::command]
pub fn save_character(directory: String, character: Character) -> Result<(), String> {
    let path = PathBuf::from(directory).join(format!("{}.toml", character.name));
    let contents = toml::to_string(&character).map_err(|e| e.to_string())?;
    fs::write(path, contents).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn select_directory() -> Result<String, String> {
    let selected_dir = FileDialogBuilder::new()
        .pick_folder()
        .ok_or_else(|| "No directory selected".to_string())?;

    selected_dir.into_os_string().into_string().map_err(|e| e.to_string_lossy().into_owned())
}