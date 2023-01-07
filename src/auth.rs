use crate::{state::{DataState, Auth, User}, defs};
use sha2::{Sha512, Digest};

fn get_salted_pw(password: String) -> String {
    return password + defs::HASH_SALT;
}

fn get_sha512_base64(str: String) -> String {
    let mut hasher = Sha512::new();
    let hasher_ref = &mut hasher;
    hasher_ref.update(str);
    return base64::encode(&hasher.finalize());
}

pub(crate) fn verify_auth(sat: String, data_state: &mut DataState) -> u64 {
    let data_state_auth_ref = &mut data_state.auth;
    for rec in data_state_auth_ref {
        if sat == rec.token {
            return rec.uid;
        }
    }
    return 0;
}

pub(crate) fn auth(uid: u64, password: String, data_state: &mut DataState) -> String {
    if uid > 0 {
        let data_state_user_ref = &mut data_state.user;
        for rec in data_state_user_ref {
            if uid == rec.id {
                let salted_pw = get_salted_pw(password);
                println!("salted_pw {}", salted_pw);
                let pw_hash_base64 = get_sha512_base64(salted_pw);
                if pw_hash_base64 == rec.password {
                    let rand0: u64 = rand::random();
                    let rand1: u64 = rand::random();
                    let mut term: u64 = 0;
                    loop {
                        let token = get_sha512_base64((uid.overflowing_mul(rand0).0.overflowing_add(rand1).0.overflowing_add(term).0).to_string());
                        let mut is_unique = true;
                        let data_state_auth_ref = &mut data_state.auth;
                        for auth_rec in data_state_auth_ref {
                            if token == auth_rec.token {
                                is_unique = false;
                                break;
                            }
                        }
                        if is_unique {
                            data_state.auth.push(Auth{token: token.clone(), uid});
                            return token;
                        } else {
                            term += 1;
                        }
                    }
                }
                break;
            }
        }
    }
    return "".to_string();
}

pub(crate) fn reg(name: String, password: String, password_repeat: String, data_state: &mut DataState) -> u64 {
    if !name.is_empty() && password.len() >= defs::PASSWORD_MIN_LENGTH && password == password_repeat {
        let pw_hash = get_sha512_base64(get_salted_pw(password));
        let mut id = 0;
        let data_state_user_ref = &mut data_state.user;
        for rec in data_state_user_ref {
            if rec.id > id {
                id = rec.id;
            }
        }
        id += 1;
        data_state.user.push(User{id, name, password: pw_hash});
        return id;
    }
    return 0;
}

pub(crate) fn logout(sat: String, data_state: &mut DataState) -> bool {
    let data_state_auth_ref = &mut data_state.auth;
    for i in 0..data_state_auth_ref.len() {
        if data_state_auth_ref.get(i).unwrap().token == sat {
            data_state_auth_ref.remove(i);
            return true;
        }
    }
    return false;
}