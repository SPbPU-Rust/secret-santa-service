use std::time::{ SystemTime, UNIX_EPOCH };


#[derive(PartialEq, Eq)]
pub enum Types {
    User,
    Group,
}


pub async fn get_id(kind: Types) -> u64 {
    let mut id = ((SystemTime::now().duration_since(UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_nanos() / 1000) as u64).to_string();
    id += if kind == Types::User {
        "0"
    } else {
        "1"
    };
    id.parse::<u64>().unwrap_or_default()
}
