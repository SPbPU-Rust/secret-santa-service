#[path = "group.rs"] mod group;
pub use group::Group;
use group::ip::Types;
use group::ip;


#[derive(Debug)]
pub struct User {
    id: u64,
    nickname: String,
    password: String,
    groups: Vec<u64>,
}


impl User {
    pub async fn new() -> User {
        User {
            id: ip::get_id(Types::User).await,
            nickname: String::new(),
            password: String::new(),
            groups: Vec::new(),
        }
    }
    pub async fn new_with_fields(nickname: String, password: String) -> User {
        User {
            id: ip::get_id(Types::User).await,
            nickname,
            password,
            groups: Vec::new(),
        }
    }
    pub async fn set_id(&mut self, id: u64) {
        self.id = id
    }
    pub async fn set_nickname(&mut self, name: String) {
        self.nickname = name
    }
    pub async fn set_password(&mut self, password: String) {
        self.password = password
    }
    pub async fn get_id(&self) -> u64 {
        self.id
    }
    pub async fn get_nickname(&self) -> &String {
        &self.nickname
    }
    pub async fn get_password(&self) -> &String {
        &self.password
    }
    pub async fn get_groups(&self) -> &Vec<u64> {
        &self.groups
    }
    pub async fn print_info(&self) {
        println!("\nUSER INFO:\n{}\n{}\n{}\n{:?}\n", self.id, self.nickname, self.password, self.groups)
    }
    pub async fn create_group(&mut self) -> Group {
        let new_group: Group = Group::new(self.id).await;
        self.groups.push(new_group.get_id().await);
        
        new_group.print_info().await;
        new_group
    }
}
