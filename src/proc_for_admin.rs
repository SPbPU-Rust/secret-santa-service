fn make_new_admin(aid: u64, uid: u64, gid: u64, data_state: &mut DataState) {
    for cid in data_state.user_in_group {
        if cid.uid == uid && cid.gid == gid {
            cid.is_admin = true;
            return;
        }
    }
}

fn remove_admin_rights(aid: u64, id: u64, gid: u64, data_state: &mut DataState) {
    for cid in data_state.user_in_group {
        if cid.uid == aid && cid.gid == gid {
            if cid.is_admin = true {
                for nid in data_state.user_in_group {
                    if nid.uid == id && nid.gid == gid && check_admins_number(gid, data_state) > 1 {
                        cid.is_admin = false;
                        return;
                    }
                }
            }
        }
    }
}

fn leave_group(aid: u64, gid: u64, data_state: &mut DataState) {
    if check_admins_number(gid, data_state) > 1 {
        for cid in data_state.user_in_group {
            if cid.uid == aid && cid.is_admin == true {
                data_state.user_in_group.remove(cid);
                return;
            }
        }
    }
}

fn remove_group(aid: u64, gid: u64, data_state: &mut DataState) {
    for cg in data_state.group {
        if cg.id == gid {
            data_state.group.remove(cg);
            return;
            /*удалить всех пользователей этой группы из user_in_group */
        }
    }
}

fn distr_sec_santas(aid: u64, gid: u64, data_state: &mut DataState) {
    /* */
    for cg in data_state.group {
        if cg.id == gid {
            cg.is_closed = true;
        }
    }
}

fn check_admins_number(gid: u64, data_state: &mut DataState) -> u64 {
    let count: u64 = 0;
    for cid in data_state.user_in_group {
        if cid.gid == gid && cid.is_admin {
            count = count + 1;
        }
    }
    return count;
}
