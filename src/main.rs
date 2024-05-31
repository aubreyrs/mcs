mod module;

use module::skin;
use clap::Parser;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    username: String,
}

#[derive(Deserialize)]
struct MojangProfile {
    id: String,
    name: String,
}

fn get_uuid(username: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://api.mojang.com/users/profiles/minecraft/{}", username);
    let client = Client::new();
    let response = client.get(&url).send()?;
    let profile: MojangProfile = response.json()?;
    Ok(profile.id)
}

fn main() {
    let cli = Cli::parse();
    let player_name = cli.username;

    match get_uuid(&player_name) {
        Ok(uuid) => {
            match skin::render_head_with_info(&player_name, &uuid) {
                Ok(_) => (),
                Err(e) => eprintln!("Error rendering head: {}", e),
            }
        },
        Err(e) => eprintln!("Error fetching UUID: {}", e),
    }
}
