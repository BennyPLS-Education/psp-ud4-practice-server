use crate::storage;
use crate::storage::load;
use serde::{Deserialize, Serialize};
use std::io;
use std::io::ErrorKind::NotFound;
use std::path::PathBuf;

const FILE: &str = "Videojocs_DB.txt";

/// Represents a video game creation object.
///
/// This struct is used to create a new video game.
#[derive(Debug, Deserialize)]
pub struct VideoGameCreate {
    #[serde(rename = "TITOL")]
    title: String,
    #[serde(rename = "ANY")]
    year: String,
    #[serde(rename = "MODALITAT")]
    mode: String,
    #[serde(rename = "EMPRESA")]
    company: String,
}

/// Represents a video game object.
#[derive(Debug, Deserialize, Serialize)]
pub struct VideoGame {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "TITOL")]
    pub title: String,
    #[serde(rename = "ANY")]
    pub year: String,
    #[serde(rename = "MODALITAT")]
    pub mode: String,
    #[serde(rename = "EMPRESA")]
    pub company: String,
}

impl VideoGame {
    /// Converts a `VideoGameCreate` struct into a `VideoGame` struct.
    ///
    /// The id field is automatically generated.
    pub fn from(game: VideoGameCreate) -> VideoGame {
        VideoGame {
            id: get_next_id(),
            title: game.title,
            year: game.year,
            mode: game.mode,
            company: game.company,
        }
    }
}

/// Initializes the video game database.
pub fn initialize() {
    let games = get_data();
    if games.is_empty() {
        let games = vec![
            VideoGame {
                id: 1,
                title: "The Legend of Zelda: Breath of the Wild".to_string(),
                year: "2017".to_string(),
                mode: "Aventura".to_string(),
                company: "Nintendo".to_string(),
            },
            VideoGame {
                id: 2,
                title: "The Witcher 3: Wild Hunt".to_string(),
                year: "2015".to_string(),
                mode: "Rol".to_string(),
                company: "CD Projekt".to_string(),
            },
            VideoGame {
                id: 3,
                title: "Red Dead Redemption 2".to_string(),
                year: "2018".to_string(),
                mode: "Aventura".to_string(),
                company: "Rockstar".to_string(),
            },
            VideoGame {
                id: 4,
                title: "The Elder Scrolls V: Skyrim".to_string(),
                year: "2011".to_string(),
                mode: "Rol".to_string(),
                company: "Bethesda".to_string(),
            },
            VideoGame {
                id: 5,
                title: "Grand Theft Auto V".to_string(),
                year: "2013".to_string(),
                mode: "Aventura".to_string(),
                company: "Rockstar".to_string(),
            },
        ];
        save_data(&games).unwrap();
    }
}

/// Loads the video game database.
fn get_data() -> Vec<VideoGame> {
    let data = load(&PathBuf::from(FILE));
    serde_json::from_str(&data).unwrap_or(Vec::new())
}

/// Saves the video game database.
fn save_data(games: &Vec<VideoGame>) -> Result<(), io::Error> {
    let data = serde_json::to_string(&games).unwrap();
    storage::save(&data, &PathBuf::from(FILE))
}

/// Creates a new video game.
pub fn create_game(game: VideoGame) -> Result<(), io::Error> {
    let mut games = get_data();
    games.push(game);
    save_data(&games)
}

/// Reads all video games.
pub fn read_games() -> Vec<VideoGame> {
    get_data()
}

/// Reads a video game by its id.
pub fn read_game_by_id(id: i32) -> Option<VideoGame> {
    get_data().into_iter().find(|game| game.id == id)
}

/// Reads a video game by its corporation.
pub fn read_game_by_corporation(corporation: &str) -> Vec<VideoGame> {
    get_data()
        .into_iter()
        .filter(|game| game.company == corporation)
        .collect()
}

/// Updates a video game by its id.
pub fn update_game(id: i32, game: VideoGame) -> io::Result<()> {
    let mut games = get_data();
    let index = games.iter().position(|g| g.id == id);

    match index {
        Some(index) => {
            games[index] = game;
            save_data(&games)
        }
        None => Err(io::Error::new(NotFound, "Game not found")),
    }
}

/// Deletes a video game by its id.
pub fn delete_game(id: i32) -> Result<(), io::Error> {
    let mut games = get_data();
    games.retain(|game| game.id != id);
    save_data(&games)
}

/// Gets the next id for a new video game.
pub fn get_next_id() -> i32 {
    let games = get_data();
    if games.is_empty() {
        1
    } else {
        games.iter().map(|game| game.id).max().unwrap() + 1
    }
}
