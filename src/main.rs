mod database;
mod error_handlers;
mod storage;

use crate::database::{VideoGame, VideoGameCreate};
use rocket::serde::json::Json;
use rocket::{Build, Rocket};

#[macro_use]
extern crate rocket;

/// HTTP handler for getting all video games.
#[get("/")]
async fn get_all_video_games() -> Json<Vec<VideoGame>> {
    let games = database::read_games();

    Json(games)
}

/// HTTP handler for getting a video game by ID.
#[get("/<id>")]
async fn get_game_by_id(id: i32) -> Result<Json<VideoGame>, Json<String>> {
    let game = database::read_game_by_id(id);

    match game {
        Some(game) => Ok(Json(game)),
        None => Err(Json("Game not found".to_string())),
    }
}

/// HTTP handler for getting a video game by corporation.
#[get("/empresa/<corporation>")]
async fn get_game_by_corporation(corporation: String) -> Json<Vec<VideoGame>> {
    let games = database::read_game_by_corporation(&corporation);

    Json(games)
}

/// HTTP handler for creating a video game.
#[post("/", data = "<game>")]
async fn create_game(game: Json<VideoGameCreate>) -> Json<String> {
    match database::create_game(VideoGame::from(game.into_inner())) {
        Ok(_) => Json("Game created".to_string()),
        Err(_) => Json("Error creating game".to_string()),
    }
}

/// HTTP handler for updating a video game.
#[put("/update", data = "<game>")]
async fn update_game(game: Json<VideoGame>) -> Json<String> {
    let game = game.into_inner();
    match database::update_game(game.id, game) {
        Ok(_) => Json("Game updated".to_string()),
        Err(_) => Json("Error updating game".to_string()),
    }
}

/// HTTP Handler for deleting a game.
#[delete("/delete/<id>")]
async fn delete_game(id: i32) -> Json<String> {
    match database::delete_game(id) {
        Ok(_) => Json("Game deleted".to_string()),
        Err(_) => Json("Error deleting game".to_string()),
    }
}

/// Main function to start the Rocket application.
#[launch]
fn rocket() -> Rocket<Build> {
    database::initialize();

    rocket::build()
        .register(
            "/",
            catchers![
                error_handlers::not_found,
                error_handlers::unprocessable_entity
            ],
        )
        .mount(
            "/videojocs",
            routes![
                get_all_video_games,
                get_game_by_id,
                get_game_by_corporation,
                create_game,
                update_game,
                delete_game
            ],
        )
}
