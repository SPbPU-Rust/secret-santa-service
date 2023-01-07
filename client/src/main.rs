mod args;

use serde_json::value::Value;
use clap::Parser;
use std::str;
use std::collections::HashMap;
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
        EntityType::Create(create_room) => {
            let mut map: HashMap<&str, _> = HashMap::new();
            map.insert("admin_token", create_room.admin_token);
            let request = format!("{addr}/create_room");
            let client = reqwest::Client::new();
            let _resp = client.post(request).json(&map).send().await?.json::<serde_json::Value>().await?;
            println!("ID комнаты: {:?}", _resp.get("$oid").and_then(Value::as_str).unwrap());
        }
        EntityType::Get(get_room) => {
            let room_id = get_room.room_id;
            let name = get_room.name;
            let request = format!("{addr}/room/{room_id}/{name}");
            let _resp = reqwest::get(request).await?.json::<serde_json::Value>().await?;
            println!("{:?}", _resp);
        }
        EntityType::Start(start_game) => {
            let room_id = start_game.room_id;
            let admin_token = start_game.admin_token;
            let request = format!("{addr}/start/{room_id}/{admin_token}");
            let _resp = reqwest::get(request).await?.json::<serde_json::Value>().await?;
            println!("{:?}", _resp);
        }
        EntityType::End(end_game) => {
            let room_id = end_game.room_id;
            let admin_token = end_game.admin_token;
            let request = format!("{addr}/end/{room_id}/{admin_token}");
            let _resp = reqwest::get(request).await?.json::<serde_json::Value>().await?;
            println!("{:?}", _resp);
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
