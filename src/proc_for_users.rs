use crate::state::{DataState, Group, UserInGroup};

pub(crate) fn make_group(creator_uid: u64, name: String, data_state: &mut DataState) -> u64 {
    if creator_uid == 0 {
        return 0;
    }
    let mut data_state_group_ref = &mut data_state.group;
    let mut new_group_id: u64 = 0;
    for rec in data_state_group_ref {
        if rec.id > new_group_id {
            new_group_id = rec.id;
        }
    }
    new_group_id += 1;
    data_state_group_ref = &mut data_state.group;
    data_state_group_ref.push(Group{id: new_group_id, name, is_closed: false});
    let data_state_uig_ref = &mut data_state.user_in_group;
    data_state_uig_ref.push(UserInGroup{uid: creator_uid, gid: new_group_id, is_admin: true, santa_for: 0});
    return new_group_id;
}

// В каких группах состоит пользователь
pub(crate) fn list_of_users_groups(uid: u64, data_state: &mut DataState) -> Vec<u64> {
    let mut v = Vec::<u64>::new();
    let data_state_uig_ref = &mut data_state.user_in_group;
    for c in data_state_uig_ref {
        if uid == c.uid {
            v.push(c.gid);
        }
    }
    return v;
}

// Список пользователей группы
pub(crate) fn list_users_in_group(gid: u64, data_state: &mut DataState) -> Vec<u64> {
    let mut v = Vec::<u64>::new();
    let data_state_uig_ref = &mut data_state.user_in_group;
    for c in data_state_uig_ref {
        if gid == c.gid {
            v.push(c.uid);
        }
    }
    return v;
}
// Список пользователей группы (возврат записей UserInGroup)
pub(crate) fn list_users_in_group_recs(gid: u64, data_state: &mut DataState) -> Vec<&mut UserInGroup> {
    let mut v = Vec::<&mut UserInGroup>::new();
    let data_state_uig_ref = &mut data_state.user_in_group;
    for c in data_state_uig_ref {
        if gid == c.gid {
            v.push(c);
        }
    }
    return v;
}

// Найти, для кого пользователь uid является Сантой в группе gid
pub(crate) fn find_out_ss(uid: u64, gid: u64, data_state: &mut DataState) -> u64 {
    if (gid > 0) {
        let data_state_uig_ref = &mut data_state.user_in_group;
        for c in data_state_uig_ref {
            if c.uid == uid && c.gid == gid {
                /*if cid.santa_for == 0 {
                    let s = String::from("Santa for no one");
                    return s;
                } else {
                    for ss in data_state.user {
                        if ss.id == santa_for {
                            let s = String::from("You are Santa for {}, id: {}", ss.name, santa_for);
                            return s;
                        }
                    }
                }*/
                return c.santa_for;
            }
        }
    }
    return 0;
}

// Вступить в группу
pub(crate) fn join_group(uid: u64, gid: u64, data_state: &mut DataState) -> bool {
    /*for cid in data_state.group {
        if cid.id == gid {
            if cid.is_closed == true {
                let s = String::from("this group is closed");
                return s;
            } else {
                let new_user_in_group = UserInGroup(id, gid, false, 0);
                user_in_group.push(new_user_in_group);
                let s = String::from("user {} joined group", id, gid);
            }
        }
    }*/
    let data_state_group_ref = &mut data_state.group;
    for cid in data_state_group_ref {
        if cid.id == gid {
            if cid.is_closed == true {
                return false;
            } else {
                let mut data_state_uig_ref = &mut data_state.user_in_group;
                for uig_rec in data_state_uig_ref {
                    if uig_rec.gid == gid && uig_rec.uid == uid {
                        return true; // уже был присоединен к группе
                    }
                }
                data_state_uig_ref = &mut data_state.user_in_group;
                data_state_uig_ref.push(UserInGroup{uid, gid, is_admin: false, santa_for: 0});
                return true;
            }
        }
    };
    return false;
}
