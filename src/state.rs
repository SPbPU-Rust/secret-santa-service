// Структуры данных
pub(crate) struct Auth {
    token: String,
    uid: u64
}
pub(crate) struct User {
    id: u64,
    name: String,
    password: String
}
pub(crate) struct Group {
    id: u64,
    name: String,
    is_closed: bool
}
pub(crate) struct UserInGroup {
    uid: u64,
    gid: u64,
    is_admin: bool,
    santa_for: u64
}