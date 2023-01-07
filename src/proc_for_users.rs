fn make_group(id: u64, name: String, data_state: &mut DataState) -> u64 {
    let gid: u64 = group.len() + 1;
    let new_group = Group(gid, name, false);
    group.push(new_group);
    let new_user_in_group = UserInGroup(id, gid, true, 0);
    user_in_group.push(new_user_in_group);
    return gid;
}

fn list_of_users_groups(id: u64, data_state: &mut DataState) -> Vec<u64> {
    let v = Vec::<u64>;
    for cid in data_state.user_in_group {
        if id == cid.uid {
            v.push(cid.gid);
        }
    }
    return v;
}

fn find_out_ss(id: u64, gid: u64, data_state: &mut DataState) -> String {
    for cid in data_state.user_in_group {
        if cid.uid == id && cid.gid == gid {
            if santa_for == 0 {
                let s = String::from("Santa for no one");
                return s;
            } else {
                for ss in data_state.user {
                    if ss.id == santa_for {
                        let s = String::from("You are Santa for {}, id: {}", ss.name, santa_for);
                        return s;
                    }
                }
            }
        }
    }
}

fn join_group(id: u64, gid: u64, data_state: &mut DataState) -> String {
    for cid in data_state.group {
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
    }
}
