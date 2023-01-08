use crate::state::DataState;

fn is_group_admin(uid: u64, gid: u64, data_state: &mut DataState) -> bool {
    let data_stat_uig_ref = &mut data_state.user_in_group;
    for rec in data_stat_uig_ref {
        if rec.uid == uid && rec.gid == gid {
            return rec.is_admin;
        }
    }
    return false;
}

// сколько админов в группе
pub(crate) fn check_admins_number(gid: u64, data_state: &mut DataState) -> u64 {
    let mut count:u64 = 0;
    let data_state_uig_ref = &data_state.user_in_group;
    for cid in data_state_uig_ref {
        if cid.gid == gid && cid.is_admin {
            count += 1;
        }
    }
    return count;
}

// дать админку
pub(crate) fn make_new_admin(aid: u64, target_uid: u64, gid: u64, data_state: &mut DataState) -> bool {
    if (is_group_admin(aid, gid, data_state)) {
        let data_state_uig_ref = &mut data_state.user_in_group;
        for c in data_state_uig_ref {
            if c.uid == target_uid && c.gid == gid {
                c.is_admin = true;
                return true;
            } 
        }
    }
    return false;
}

// снять админку
pub(crate) fn remove_admin_rights(aid:u64, target_aid: u64, gid: u64, data_state: &mut DataState) -> bool {
    if is_group_admin(aid, gid, data_state) {
        let gt1_admins = check_admins_number(gid, data_state) > 1;
        for nid in 0..data_state.user_in_group.len() {
            let data_state_uig_ref = &mut data_state.user_in_group;
            let n = data_state_uig_ref.get_mut(nid).unwrap();
            if n.uid == target_aid && n.gid == gid && gt1_admins {
                n.is_admin = false;
                return true;
            }
        }
    }
    return false;
}

pub(crate) fn leave_group(aid: u64, gid: u64, data_state: &mut DataState) {
    if check_admins_number(gid, data_state) > 1 {
        for cid in (0..data_state.user_in_group.len()).rev() {
            let rec = data_state.user_in_group.get(cid).unwrap();
            if rec.uid == aid && rec.is_admin == true {
                data_state.user_in_group.remove(cid);
                return;
            }
        }
    }
}

pub(crate) fn remove_group(aid: u64, gid: u64, data_state: &mut DataState) -> bool {
    if is_group_admin(aid, gid, data_state) {
        for cid in 0..data_state.group.len() {
            if data_state.group.get(cid).unwrap().id == gid {
                //удаление всех пользователей этой группы из user_in_group
                for uigid in (0..data_state.user_in_group.len()).rev() {
                    let rec = data_state.user_in_group.get(uigid).unwrap();
                    if rec.gid == gid {
                        data_state.user_in_group.remove(uigid);
                    }
                }
                data_state.group.remove(cid);
                return true;
            }
        }
    }
    return false;
}

// запустить секретных сант, закрыть группу
pub(crate) fn distr_sec_santas(aid: u64, gid: u64, data_state: &mut DataState) -> bool {
    if is_group_admin(aid, gid, data_state) {
        let data_state_group_ref = &mut data_state.group;
        for cg in data_state_group_ref {
            if cg.id == gid {
                
                //распределение
                let mut users_of_group = Vec<UserInGroup>;
                for cu in data_state.user_in_group {
                    if cu.gid == gid {
                        users_of_group.push(cu);
                    }
                }

                let mut v_digits: [u64; users_of_group.len()];
                for i in 0..users_of_group.len() {
                    v_digits[i] = i;
                }

                v_digits.swap(0, users_of_group.len() / 2);

                for j in 0..users_of_group.len() / 2 {
                    v_digits.swap(0, users_of_group.len() / 2 - j);
                }

                for change in users_of_group.len() {
                    users_of_group[change].santa_for = users_of_group[v_digits[change]].uid;
                }
                cg.is_closed = true;
                return true;
            }
        }
    }
    return false;
}
