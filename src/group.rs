#[path = "ip.rs"] pub mod ip;
pub use ip::Types;


#[derive(Debug)]
enum Access {
    Public,
    Private,
}


pub struct Group {
    id: u64,
    access: Access,
    members: Vec<(u64, bool)>,
}


impl Group {
    pub async fn new(admin: u64) -> Group {
        Group {
            id: ip::get_id(Types::Group).await,
            access: Access::Public,
            members: vec![(admin, true)],
        }
    }
    pub async fn get_id(&self) -> u64 {
        self.id
    }
    pub async fn print_info(&self) {
        println!("\nGROUP INFO\n{}\n{:?}\n{:?}\n", self.id, self.access, self.members);
    }
}
