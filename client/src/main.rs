mod args;

use clap::Parser;
use std::error::Error;
use std::env;

use crate::args::EntityType;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = args::SantaArgs::parse();
    env::set_var("ADDR_SERVER", "http://127.0.0.1:8080");
    let addr = env::var("ADDR_SERVER").unwrap();

    match args.entity_type {
        EntityType::Hi(_hello) => {
            let request = format!("{addr}");
            let _resp = reqwest::get(request).await?.json::<serde_json::Value>().await?;
            println!("{:?}", _resp.as_str().unwrap())
        }
        EntityType::Duser(delete_user) => {
            let name = delete_user.name;
            let room_id = delete_user.room_id;
            let admin_token = delete_user.admin_token;
            let request = format!("{addr}/room/{room_id}/{name}/{admin_token}");
            let client = reqwest::Client::new();
            let _resp = client.delete(request).send().await?.json::<serde_json::Value>().await?;
            println!("{:?}", _resp);
        }
        EntityType::Droom(delete_room) => {
            let room_id = delete_room.room_id;
            let admin_token = delete_room.admin_token;
            let request = format!("{addr}/room/{room_id}/{admin_token}");
            let client = reqwest::Client::new();
            let _resp = client.delete(request).send().await?.json::<serde_json::Value>().await?;
            println!("{:?}", _resp);
        }
    }
    Ok(())
}
