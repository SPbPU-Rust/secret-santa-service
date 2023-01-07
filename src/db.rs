use std::io::{Write, Read};

use sqlx::{self, Error, SqlitePool, Row};
use rand::Rng;

#[path = "user.rs"] pub mod user;
pub use user::{User, Group};


static DATABASE_URL: &str = "info.db";


pub async fn check_tables() -> Result<(), Error> {
    match std::fs::File::open(&DATABASE_URL) {
        Ok(_) => (),
        Err(_) => { std::fs::File::create(&DATABASE_URL)?; },
    };
    
    let pool = SqlitePool::connect(&DATABASE_URL).await?;

    sqlx::query("
        CREATE TABLE IF NOT EXISTS users (
            id BIGINT,
            nickname TEXT,
            password TEXT
    )").execute(&pool).await?;

    sqlx::query("
        CREATE TABLE IF NOT EXISTS groups (
            id BIGINT,
            access BOOLEAN
    )").execute(&pool).await?;

    sqlx::query("
    CREATE TABLE IF NOT EXISTS users_groups (
        userId BIGINT,
        groupId BIGINT,
        isAdmin BOOLEAN
    )").execute(&pool).await?;
    
    pool.close().await;
    Ok(())
}


pub async fn register_user(user: &User) -> Result<bool, Error> {
    if is_registered(&user).await? != 0 {
        return Ok(false)
    }

    let pool = SqlitePool::connect(&DATABASE_URL).await.unwrap();
    let qry = "INSERT INTO users (id, nickname, password) VALUES($1, $2, $3)";
    sqlx::query(&qry).bind(user.get_id().await as i64)
                          .bind(user.get_nickname().await)
                          .bind(user.get_password().await)
                          .execute(&pool).await?;

    pool.close().await;
    Ok(true)
}


pub async fn is_registered(user: &User) -> Result<i64, Error> {
    let pool = SqlitePool::connect(&DATABASE_URL).await?;

    let qry = "SELECT id FROM users WHERE nickname = $1 AND password = $2";
    let user_exists = sqlx::query(&qry).bind(&user.get_nickname().await)
                                       .bind(&user.get_password().await)
                                       .fetch_all(&pool).await?;
    if user_exists.len() == 0 {
        Ok(0)
    } else {
        Ok(user_exists[0].try_get(0)?)
    }
}


pub async fn get_available_groups(mut vec: Vec<u64>) -> Result<Vec<u64>, Error> {
    let pool = SqlitePool::connect(&DATABASE_URL).await?;

    let qry = "SELECT * FROM groups WHERE access = True";
    let groups = sqlx::query(&qry).fetch_all(&pool).await?;
    for group in groups {
        let id: i64 = group.try_get(0)?;
        vec.push(id as u64);
    }
    Ok(vec)
}


pub async fn get_user_groups(user_id: u64, mut vec: Vec<u64>) -> Result<Vec<u64>, Error> {
    let pool = SqlitePool::connect(&DATABASE_URL).await?;

    let qry = "SELECT * FROM users_groups WHERE userId = $1";
    let groups = sqlx::query(&qry).bind(user_id as i64)
                                  .fetch_all(&pool).await?;
    for group in groups {
        let id: i64 = group.try_get(1)?;
        vec.push(id as u64);
    }
    Ok(vec)
}


pub async fn add_group(mut user: User) -> Result<Group, Error> {
    let group: Group = user.create_group().await;
    let pool = SqlitePool::connect(&DATABASE_URL).await?;

    let qry = "INSERT INTO groups (id, access) VALUES($1, True)";
    sqlx::query(&qry).bind(group.get_id().await as i64)
                     .execute(&pool).await?;

    let qry = "INSERT INTO users_groups (userId, groupId, isAdmin) VALUES($1, $2, True)";
    sqlx::query(&qry).bind(user.get_id().await as i64)
                     .bind(group.get_id().await as i64)
                     .execute(&pool).await?;

    pool.close().await;
    Ok(group)
}


pub async fn join_group(user_id: u64, group_id: u64) -> Result<bool, Error> {
    let pool = SqlitePool::connect(&DATABASE_URL).await?;

    let qry = "SELECT * FROM users_groups WHERE userId = $1 AND groupId = $2";
    let joined = !sqlx::query(&qry).bind(user_id as i64)
                                  .bind(group_id as i64)
                                  .fetch_all(&pool).await?.is_empty();
    if joined {
        pool.close().await;
        return Ok(false)
    }

    let qry = "SELECT * FROM groups WHERE id = $1";
    let access: bool = sqlx::query(&qry).bind(group_id as i64)
                                  .fetch_one(&pool).await?.try_get(1)?;
    
    if !access {
        pool.close().await;
        return Ok(false)
    }

    let qry = "INSERT INTO users_groups (userId, groupId, isAdmin) VALUES($1, $2, False)";
    sqlx::query(&qry).bind(user_id as i64).bind(group_id as i64).execute(&pool).await?;

    pool.close().await;
    Ok(true)
}


pub async fn leave_group(user_id: u64, group_id: u64) -> Result<bool, Error> {
    let pool = SqlitePool::connect(&DATABASE_URL).await?;

    let qry = "SELECT * FROM users_groups WHERE userId = $1 AND groupId = $2 AND isAdmin = False";
    let can_leave = !sqlx::query(&qry).bind(user_id as i64)
                                  .bind(group_id as i64)
                                  .fetch_all(&pool).await?.is_empty();

    if !can_leave {
        pool.close().await;
        return Ok(false)
    }

    let qry = "SELECT * FROM groups WHERE id = $1";
    let access: bool = sqlx::query(&qry).bind(group_id as i64)
                                  .fetch_one(&pool).await?.try_get(1)?;
    
    if !access {
        pool.close().await;
        return Ok(false)
    }

    let qry = "DELETE FROM users_groups WHERE userId = $1 AND groupId = $2";
    sqlx::query(&qry).bind(user_id as i64).bind(group_id as i64).execute(&pool).await?;

    pool.close().await;
    Ok(true)
}


pub async fn make_admin(user_id: u64, group_id: u64, other_user_id: u64) -> Result<bool, Error> {
    let pool = SqlitePool::connect(&DATABASE_URL).await?;

    if !is_admin(user_id, group_id).await? {
        pool.close().await;
        return Ok(false)
    }

    let qry = "SELECT * FROM users_groups WHERE userId = $1 AND groupId = $2";
    let is_member = !sqlx::query(&qry).bind(other_user_id as i64)
                                  .bind(group_id as i64)
                                  .fetch_all(&pool).await?.is_empty();
    if !is_member {
        pool.close().await;
        return Ok(false)
    }

    let qry = "UPDATE users_groups SET isAdmin = True WHERE userId = $1 AND groupId = $2";
    sqlx::query(&qry).bind(other_user_id as i64)
                     .bind(group_id as i64).execute(&pool).await?;

    pool.close().await;
    Ok(true)
}


pub async fn un_administer(user_id: u64, group_id: u64) -> Result<bool, Error> {
    let pool = SqlitePool::connect(&DATABASE_URL).await?;

    if !is_admin(user_id, group_id).await? {
        pool.close().await;
        return Ok(false)
    }

    let qry = "SELECT * FROM users_groups WHERE groupId = $1 AND isAdmin = True";
    let admins_count = sqlx::query(&qry).bind(group_id as i64)
                                  .fetch_all(&pool).await?.len();
    if admins_count < 2 {
        pool.close().await;
        return Ok(false)
    }

    let qry = "UPDATE users_groups SET isAdmin = False WHERE userId = $1 AND groupId = $2";
    sqlx::query(&qry).bind(user_id as i64)
                     .bind(group_id as i64).execute(&pool).await?;

    pool.close().await;
    Ok(true)
}


pub async fn is_admin(user_id: u64, group_id: u64) -> Result<bool, Error> {
    let pool = SqlitePool::connect(&DATABASE_URL).await?;

    let qry = "SELECT * FROM users_groups WHERE userId = $1 AND groupId = $2";
    let admin = sqlx::query(&qry).bind(user_id as i64)
                                  .bind(group_id as i64)
                                  .fetch_one(&pool).await?;
    pool.close().await;
    Ok(admin.try_get(2)?)
}


pub async fn drop_group(user_id: u64, group_id: u64) -> Result<bool, Error> {
    let pool = SqlitePool::connect(&DATABASE_URL).await?;

    if !is_admin(user_id, group_id).await? {
        pool.close().await;
        return Ok(false)
    }

    let qry = "DELETE FROM users_groups WHERE groupId = $1";
    sqlx::query(&qry).bind(group_id as i64)
                     .execute(&pool).await?;

    let qry = "DELETE FROM groups WHERE id = $1";
    sqlx::query(&qry).bind(group_id as i64)
                     .execute(&pool).await?;

    pool.close().await;
    Ok(true)
}


pub async fn get_group_members(mut vec: Vec<(u64, bool)>, group_id: u64) -> Result<Vec<(u64, bool)>, Error> {
    let pool = SqlitePool::connect(&DATABASE_URL).await?;

    let qry = "SELECT * FROM users_groups WHERE groupId = $1";
    let members = sqlx::query(&qry).bind(group_id as i64)
                                  .fetch_all(&pool).await?;
    for member in members {
        let id: i64 = member.try_get(0)?;
        let is_admin = member.try_get(2)?;
        vec.push((id as u64, is_admin));
    }                
    Ok(vec)
}


pub async fn close_group(user_id: u64, group_id: u64) -> Result<bool, Error> {
    let pool = SqlitePool::connect(&DATABASE_URL).await?;

    let qry = "SELECT * FROM groups WHERE id = $1";
    let access: bool = sqlx::query(&qry).bind(group_id as i64)
                                        .fetch_one(&pool)
                                        .await?.try_get(1)?;
    if !access {
        pool.close().await;
        return Ok(false);
    }

    if is_admin(user_id, group_id).await? {
        let qry = "UPDATE groups SET access = False WHERE id = $1";
        sqlx::query(&qry).bind(group_id as i64).execute(&pool).await?;

        pool.close().await;
        return Ok(true);
    }

    pool.close().await;
    Ok(false)
}


pub async fn set_pairs(members: Vec<(u64, bool)>, group_id: u64) -> Result<(), Error> {
    let mut fst = Vec::new();
    for i in members {
        fst.push(i.0);
    }
    let mut sec = fst.clone();
    let mut res: Vec<(u64, u64)> = Vec::new();
    for first in fst {
        let item = rand::thread_rng().gen_range(0..sec.len());
        res.push((first, sec.remove(item)));
    }

    let filename = format!("data/{}.txt", group_id);
    let mut file = std::fs::File::create(&filename)?;
    for pair in res {
        file.write(format!("{} {}\n", pair.0, pair.1).as_bytes())?;
    }

    Ok(())
}


pub async fn get_pair(user_id: u64, group_id: u64) -> Result<u64, Error> {
    let filename = format!("data/{}.txt", group_id);
    let mut file = std::fs::File::open(&filename)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    for pair in data.split('\n') {
        let ids: Vec<&str> = pair.split(' ').collect();
        if ids[0] == user_id.to_string() {
            return Ok(ids[1].parse::<u64>().unwrap_or(0));
        }
    }
    Ok(0)
}
