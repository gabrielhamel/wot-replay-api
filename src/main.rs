#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::io::Write;
use rocket::fairing::Info;
use rocket::response::content;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use wot_replay_parser::ReplayParser;
use rocket::response::status::{BadRequest, NotFound};

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct DiscordReplay {
    attachment: String,
    name: String,
    id: String,
    size: usize,
    url: String,
    proxyURL: String,
    ephemeral: bool
}

#[derive(Debug, PartialEq, Eq, Serialize)]
struct InformationsResponse {
    version: String,
    date: String,
    map: String,
    player: String
}

#[post("/", format = "application/json", data = "<discord_replay>")]
fn informations(discord_replay: Json<DiscordReplay>) -> Result<Json<InformationsResponse>, BadRequest<String>> {
    // Save the file
    let raw = reqwest::blocking::get(&discord_replay.url).expect("").bytes().expect("");
    let mut file = std::fs::File::create(&discord_replay.name).expect("");
    file.write(raw.as_ref()).expect("");

    let replay = wot_replay_parser::ReplayParser::parse_file(&discord_replay.name).unwrap();
    let replay_json_start = replay.replay_json_start().unwrap();

    let infos = InformationsResponse {
        date: String::from(replay_json_start.get("dateTime").unwrap().as_str().unwrap()),
        player: String::from(replay_json_start.get("playerName").unwrap().as_str().unwrap()),
        version: String::from(replay_json_start.get("clientVersionFromExe").unwrap().as_str().unwrap()),
        map: String::from(replay_json_start.get("mapDisplayName").unwrap().as_str().unwrap())
    };

    std::fs::remove_file(&discord_replay.name).expect("");

    Ok(Json(infos))
}

fn main() {
    rocket::ignite().mount("/informations", routes![informations]).launch();
}
