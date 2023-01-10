// Структуры данных
pub(crate) struct Auth {
    pub(crate) token: String,
    pub(crate) uid: u64
}
pub(crate) struct User {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) password: String
}
pub(crate) struct Group {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) is_closed: bool
}
pub(crate) struct UserInGroup {
    pub(crate) uid: u64,
    pub(crate) gid: u64,
    pub(crate) is_admin: bool,
    pub(crate) santa_for: u64
}

// Состояние
pub(crate) struct DataState {
    pub(crate) auth: Vec<Auth>,
    pub(crate) user: Vec<User>,
    pub(crate) user_in_group: Vec<UserInGroup>,
    pub(crate) group: Vec<Group>
}
