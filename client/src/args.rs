use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
pub struct SantaArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    Hi(Hello),
    Duser(DeleteUser),
    Droom(DeleteRoom)
}

#[derive(Debug, Args)]
pub struct Hello {
}

#[derive(Debug, Args)]
pub struct DeleteUser {
    pub room_id: String,
    pub name: String,
    pub admin_token: String
}

#[derive(Debug, Args)]
pub struct DeleteRoom {
    pub room_id: String,
    pub admin_token: String
}
