use std::string::String;
use serde_json::{Map, Value};

use crate::defs;
use crate::state::*;
use crate::auth;
use crate::proc_for_users as gr_usr;
use crate::proc_for_admin as gr_adm;

pub(crate) fn process_req(req: Map<String, Value>, data_state: &mut DataState) -> (String, String) {
    let err_lacking_fields: (String, String) = (defs::HTTP_STATUS_400.to_string(), "{\"error\": \"Не все обязательные поля заполнены значениями требуемых форматов\"}".to_string());
    let err_unsupported_op: (String, String) = (defs::HTTP_STATUS_400.to_string(), "{\"error\":\"Неподдерживаемая, невыполнимая или неверно сформированная операция\"}".to_string());
    let err_internal_error: (String, String) = (defs::HTTP_STATUS_500.to_string(), "{\"error\":\"Не удалось выполнить операцию из-за неполадок на сервере\"}".to_string());
    let err_not_found: (String, String) = (defs::HTTP_STATUS_404.to_string(), "Не удалось найти информацию о запрашиваемом объекте".to_string());
    // Перед считыванием значения по ключу обязательно проверять, что пара с желаемым ключом есть в структуре, чтобы не было вылета
    /* // шаблонные примеры - как считывать значения пар "ключ:значение"
    if req.contains_key("sat") {
        println!("sat: {}", req["sat"]);
    }
    if req.contains_key("action") {
        println!("action: {}", req["action"]);
    }*/
    // По результатам чтения и обработки req_map и выполнения в результате каких-то действий - возвращаем две строки:
    //     status_line (заголовок - статус ответа HTTP) и contents (тело ответа)
    let (mut s, mut c): (String, String) = ("".to_string(), "".to_string()); // статус HTTP-ответа; тело HTTP-ответа
    if req.contains_key("action") {
        // операции, требующие авторизацию (sat - токен)
        let err_unauthorized_op = (defs::HTTP_STATUS_401.to_string(), "{\"error\":\"Клиент не авторизован в системе\"}".to_string());
        if req.contains_key("sat") {
            if req["sat"].is_string() {
                let uid: u64 = auth::verify_auth(req["sat"].as_str().unwrap().to_string(), data_state);
                if uid == 0 {
                    (s, c) = err_unauthorized_op;
                } else {
                    (s, c) = match req["action"].as_str() {
                        Some("new_group") => {
                            match req.contains_key("name") && req["name"].is_string() {
                                true => {
                                    let gid = gr_usr::make_group(uid, req["name"].as_str().unwrap().to_string(), data_state);
                                    match gid {
                                        0 => err_internal_error,
                                        _ => (defs::HTTP_STATUS_200.to_string(), "{\"id\":".to_string() + &gid.to_string() + "}")
                                    }
                                },
                                false => err_lacking_fields
                            }
                        },
                        Some("logout") => {
                            match auth::logout(req["sat"].as_str().unwrap().to_string(), data_state) {
                                true => (defs::HTTP_STATUS_200.to_string(), "{\"msg\":\"Сеанс завершен\"}".to_string()),
                                false => (defs::HTTP_STATUS_500.to_string(), "{\"error\": \"Не найден сеанс для завершения (ошибка сервера)\"}".to_string())
                            }
                        },
                        Some("my_groups") => { // список групп пользователя
                            let gids = gr_usr::list_of_users_groups(uid, data_state);
                            let mut resp_arr: String = "{\"group_ids\":[".to_string();
                            for urec in gids {
                                resp_arr += &urec.to_string();
                            }
                            resp_arr += "null]}";
                            (defs::HTTP_STATUS_200.to_string(), resp_arr.to_string())
                        },
                        Some("santa_for") => { // узнать для кого пользователь стал сантой в группе
                            match req.contains_key("gid") && req["gid"].is_u64() {
                                true => {
                                    let gid = req["gid"].as_u64().unwrap();
                                    let santa_target_uid = gr_usr::find_out_ss(uid, gid, data_state);
                                    match santa_target_uid {
                                        0 => (defs::HTTP_STATUS_200.to_string(), "{\"msg\":\"Вы еще не назначены секретным Сантой в этой группе\"}".to_string()),
                                        _ => (defs::HTTP_STATUS_200.to_string(), "{\"santa_for\":".to_string() + &uid.to_string() + "}")
                                    }
                                },
                                false => err_lacking_fields
                            }
                        },
                        Some("join") => { // присоединиться к группе
                            match req.contains_key("gid") && req["gid"].is_u64() {
                                true => {
                                    match gr_usr::join_group(uid, req["gid"].as_u64().unwrap(), data_state) {
                                        true => (defs::HTTP_STATUS_200.to_string(), "{\"msg\":\"Вы вступили в группу\"}".to_string()),
                                        false => (defs::HTTP_STATUS_403.to_string(), "{\"error\":\"Невозможно вступить в группу\"}".to_string())
                                    }
                                },
                                false => err_lacking_fields
                            }
                        },
                        Some("leave") => { // покинуть группу (админ)
                            match req.contains_key("gid") && req["gid"].is_u64() {
                                true => {
                                    match gr_adm::leave_group(uid, req["gid"].as_u64().unwrap(), data_state) {
                                        true => (defs::HTTP_STATUS_200.to_string(), "{\"msg\":\"Вы покинули группу\"}".to_string()),
                                        false => (defs::HTTP_STATUS_403.to_string(), "{\"error\":\"Невозможно покинуть группу\"}".to_string())
                                    }
                                },
                                false => err_lacking_fields
                            }
                        },
                        Some("grant") => { // дать админку
                            match req.contains_key("gid") && req["gid"].is_u64() && req.contains_key("target_uid") && req["target_uid"].is_u64() {
                                true => {
                                    let gid = req["gid"].as_u64().unwrap();
                                    let target_uid = req["target_uid"].as_u64().unwrap();
                                    match gr_adm::make_new_admin(uid, target_uid, gid, data_state) {
                                        true => (defs::HTTP_STATUS_200.to_string(), "{\"msg\":\"Вы дали данному пользователю права администратора в группе\"}".to_string()),
                                        false => (defs::HTTP_STATUS_404.to_string(), "{\"error\":\"Не удалось дать этому пользователю права администратора в группе\"}".to_string())
                                    }
                                },
                                false => err_lacking_fields
                            }
                        },
                        Some("revoke") => { // снять админку
                            match req.contains_key("gid") && req["gid"].is_u64() && req.contains_key("target_uid") && req["target_uid"].is_u64() {
                                true => {
                                    let gid = req["gid"].as_u64().unwrap();
                                    let target_uid = req["target_uid"].as_u64().unwrap();
                                    match uid == target_uid {
                                        true => (defs::HTTP_STATUS_403.to_string(), "{\"error\":\"Нельзя снять с себя права администратора в группе, используйте операцию revoke_self\"}".to_string()),
                                        false => {
                                            match gr_adm::remove_admin_rights(uid, target_uid, gid, data_state) {
                                                true => (defs::HTTP_STATUS_200.to_string(), "{\"msg\":\"Вы сняли с данного пользователя права администратора в группе\"}".to_string()),
                                                false => (defs::HTTP_STATUS_404.to_string(), "{\"error\":\"Не удалось снять с этого пользователя права администратора в группе\"}".to_string())
                                            }
                                        }
                                    }
                                },
                                false => err_lacking_fields
                            }
                        },
                        Some("revoke_self") => { // снять админку с себя
                            match req.contains_key("gid") && req["gid"].is_u64() {
                                true => {
                                    let gid = req["gid"].as_u64().unwrap();
                                    match gr_adm::remove_admin_rights(uid, uid, gid, data_state) {
                                        true => (defs::HTTP_STATUS_200.to_string(), "{\"msg\":\"Вы сняли с себя права администратора в группе\"}".to_string()),
                                        false => (defs::HTTP_STATUS_404.to_string(), "{\"error\":\"Не удалось снять с себя права администратора в группе\"}".to_string())
                                    }
                                },
                                false => err_lacking_fields
                            }
                        },
                        Some("delete_group") => {
                            match req.contains_key("gid") && req["gid"].is_u64() {
                                true => {
                                    let gid = req["gid"].as_u64().unwrap();
                                    match gr_adm::remove_group(uid, gid, data_state) {
                                        true => (defs::HTTP_STATUS_200.to_string(), "{\"msg\":\"Группа удалена\"}".to_string()),
                                        false => (defs::HTTP_STATUS_403.to_string(), "{\"error\":\"Не удалось удалить группу\"}".to_string())
                                    }
                                },
                                false => err_lacking_fields
                            }
                        },
                        /*Some("rename_group") => {
                            match req.contains_key("gid") && req["gid"].is_u64() && req.contains_key("name") && req["name"].is_string() {
                                true => {
                                    let gid = req["gid"].as_u64().unwrap();
                                    match gr_adm::remove_admin_rights(uid, uid, gid, data_state) {
                                        true => (defs::HTTP_STATUS_200.to_string(), "{\"msg\":\"Вы сняли с себя права администратора в группе\"}".to_string()),
                                        false => (defs::HTTP_STATUS_404.to_string(), "{\"error\":\"Не удалось снять с себя права администратора в группе\"}".to_string())
                                    }
                                },
                                false => err_lacking_fields
                            }
                        },*/
                        Some("start_mission") => {
                            match req.contains_key("gid") && req["gid"].is_u64() {
                                true => {
                                    let gid = req["gid"].as_u64().unwrap();
                                    match gr_adm::distr_sec_santas(uid, gid, data_state) {
                                        true => (defs::HTTP_STATUS_200.to_string(), "{\"msg\":\"В группе произведено назначение секретных Сант, теперь группа закрытая\"}".to_string()),
                                        false => (defs::HTTP_STATUS_400.to_string(), "{\"error\":\"Не удалось произвести назначение секретных Сант. Проверьте, есть ли в группе 2 или более участников, есть ли у вас права администратора в группе, передан ли верный id группы.\"}".to_string())
                                    }
                                },
                                false => err_lacking_fields
                            }
                        },
                        _ => err_unsupported_op
                    };
                }
            } else {
                (s, c) = err_unauthorized_op;
            }
        } else {
            // операции, не требующие авторизацию (туда нельзя отправлять токен)
            (s, c) = match req["action"].as_str() {
                Some("user_info") => {
                    match req.contains_key("uid") && req["uid"].is_u64() {
                        true => {
                            let uid = req["uid"].as_u64().unwrap();
                            let data_state_user_ref = &data_state.user;
                            let mut user_found = false;
                            let mut user_name = "".to_string();
                            for urec in data_state_user_ref {
                                if urec.id == uid {
                                    user_name = urec.name.clone();
                                    user_found = true;
                                    break;
                                }
                            }
                            match user_found {
                                false => (defs::HTTP_STATUS_404.to_string(), "{\"error\":\"Не найден пользователь с таким id\"}".to_string()),
                                true => (defs::HTTP_STATUS_200.to_string(), "{\"id\":".to_string() + &uid.to_string() + ",\"name\":\"" + &user_name + "\"}")
                            }
                        },
                        false => err_lacking_fields
                    }
                },
                Some("login") => {
                    match req.contains_key("uid") && req.contains_key("password") &&
                          req["uid"].is_u64() && req["password"].is_string() {
                        true => {
                            let uid = req["uid"].as_u64().unwrap();
                            let pw = req["password"].as_str().unwrap().to_string();
                            let token = auth::auth(uid, pw, data_state);
                            match token.is_empty() {
                                true => (defs::HTTP_STATUS_401.to_string(), "{\"error\":\"Неверный или некорректно сформированный id пользователя и/или пароль\"}".to_string()),
                                false => (defs::HTTP_STATUS_200.to_string(), "{\"token\":\"".to_string() + &token + "\"}")
                            }
                        },
                        false => err_lacking_fields
                    }
                },
                Some("reg") => {
                    match req.contains_key("name") && req.contains_key("password") && req.contains_key("password_repeat") && 
                          req["name"].is_string() && req["password"].is_string() && req["password_repeat"].is_string() {
                        true => {
                            let name = req["name"].as_str().unwrap().to_string();
                            let password = req["password"].as_str().unwrap().to_string();
                            let password_repeat = req["password_repeat"].as_str().unwrap().to_string();
                            let uid = auth::reg(name, password, password_repeat, data_state);
                            match uid {
                                0 => (defs::HTTP_STATUS_400.to_string(), "{\"error\":\"Неверные значения полей, либо не совпадают пароль и повтор пароля\"}".to_string()),
                                _ => (defs::HTTP_STATUS_200.to_string(), "{\"id\":".to_string() + &uid.to_string() + "}")
                            }
                        },
                        false => err_lacking_fields
                    }
                },
                Some("group_stat") => {
                    match req.contains_key("gid") && req["gid"].is_u64() && req["gid"].as_u64().unwrap() > 0 {
                        true => {
                            let mut exists: bool = false;
                            let gid = req["gid"].as_u64().unwrap();
                            let mut group_name: String = "".to_string();
                            let mut group_is_closed: bool = false;
                            {
                                let data_state_group_ref = &data_state.group;
                                for rec in data_state_group_ref {
                                    if rec.id == gid {
                                        exists = true;
                                        group_name = rec.name.clone();
                                        group_is_closed = rec.is_closed;
                                        break;
                                    }
                                }
                            }
                            match exists {
                                true => {
                                    let urecs = gr_usr::list_users_in_group_recs(gid, data_state);
                                    let mut resp_arr: String = "{\"name\":\"".to_string() + &group_name +
                                        "\", \"is_closed\": " + &group_is_closed.to_string() + ", \"members\":[";
                                    for urec in urecs {
                                        resp_arr += &("{\"uid\":".to_string() + &urec.uid.to_string() +
                                            ", \"is_admin\": " + &urec.is_admin.to_string() + "},");
                                    }
                                    resp_arr += "null]}";
                                    (defs::HTTP_STATUS_200.to_string(), resp_arr.to_string())
                                },
                                false => err_not_found
                            }
                        },
                        false => err_lacking_fields
                    }
                },
                _ => (defs::HTTP_STATUS_403.to_string(), "{\"error\":\"Действие запрещено неавторизованным клиентам или не поддерживается\"}".to_string())
            };
        }
    } else {
        (s, c) = (defs::HTTP_STATUS_400.to_string(), "{\"error\":\"Действие не задано\"}".to_string());
    }
    return (s.to_string(), c.to_string());
}