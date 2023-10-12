use super::player::Player;
use crate::Frame;
use ratatui::{
    prelude::{Alignment, Rect},
    widgets::{block::Title, Block, BorderType, Borders, Paragraph},
};
use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

const SAVE_PATH: &str = "/YAPPY";

fn linux_save(player: &Player) {
    if let Some(home_dir) = env::var_os("HOME") {
        let save_dir = format!("{}{}", home_dir.to_string_lossy(), SAVE_PATH);
        let save = serde_json::to_string(player).unwrap();

        if !Path::new(&save_dir).exists() {
            let _ = fs::create_dir_all(&save_dir);
        }

        let full_file_path = &format!("{}/save.data", save_dir);
        let mut file = File::create(full_file_path)
            .unwrap_or_else(|_| panic!("Failed to open the file: {}", full_file_path));

        let _ = file.write_all(save.as_bytes());
    }
}

fn windows_save(_: &Player) {
    todo!()
}

pub fn write_save(player: &Player) {
    match std::env::consts::OS {
        "linux" => linux_save(player),
        "windows" => windows_save(player),
        _ => panic!("can't work with your OS lol get better + ratio"),
    }
}

fn linux_load() -> Option<Player> {
    if let Some(home_dir) = env::var_os("HOME") {
        let save_dir = format!("{}{}", home_dir.to_string_lossy(), SAVE_PATH);

        let full_file_path = &format!("{}/save.data", save_dir);
        let mut file = File::open(full_file_path).unwrap();

        let mut file_content: String = "".into();
        let _ = file.read_to_string(&mut file_content);

        let parsed: Result<Player, serde_json::Error> = serde_json::from_str(&file_content);
        match parsed {
            Ok(plr) => return Some(plr),
            Err(err) => panic!("Corrupted save file. {}", err),
        }
    }
    None
}

fn windows_load() -> Option<Player> {
    todo!()
}

pub fn load_save() -> Option<Player> {
    match std::env::consts::OS {
        "linux" => linux_load(),
        "windows" => windows_load(),
        _ => panic!("can't work with your OS lol get better + ratio"),
    }
}

pub fn calculate_bar(value: u128, max_value: u128, precision: u32) -> (String, String) {
    let percentage: u128 = value * <u32 as Into<u128>>::into(precision) / max_value;
    let mut filled = String::from("");
    let mut empty = String::from("");
    for i in 0..precision {
        if percentage > i.into() {
            filled.push('■');
        } else {
            empty.push('■');
        }
    }
    (filled, empty)
}

pub fn render_border_type(paragraph: &Paragraph, title: &str, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(Title::from(title.clone()).alignment(Alignment::Right));
    frame.render_widget(paragraph.clone().block(block), area);
}
