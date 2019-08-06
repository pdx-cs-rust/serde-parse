use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::io::Read;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
struct Item {
   transfer_status : u64,
   dismantle_permission : u64,
   bucket_hash : u64,
   quantity : u64,
   item_hash : u64,
   state : u64,
   bind_status : u64,
   lockable : bool,
   is_wrapper : bool,
   item_instance_id : String,
   location : i64,
}

#[derive(Serialize, Deserialize)]
struct EquipmentData {
    items: Vec<Item>
}

#[derive(Serialize, Deserialize)]
struct Equipment {
    privacy: u64,
    data: HashMap<String, EquipmentData>,
}

#[derive(Serialize, Deserialize)]
struct CharacterEquipment {
    #[serde(rename = "characterEquipment")]
    character_equipment: Equipment,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct APIResponse {
    error_code: i64,
    response: CharacterEquipment,
    error_status: String,
    message_data: HashMap<(), ()>,
    throttle_seconds: u64,
    message: String,
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut json = String::new();
    stdin.read_to_string(&mut json).unwrap();
    let _response: APIResponse =
        serde_json::from_str(&json).unwrap();
}
