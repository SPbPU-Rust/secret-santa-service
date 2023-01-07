fn make_group(id: u64, name: String) -> Group{
    let group = Group(1, name, false);
    /*... */
    return group;
}

fn list_of_users_groups(id: u64) -> Vec::<u64>{
    let v=Vec::<u64>;
    for cid in data_state.user_in_group {
        if id==cid.uid{
            v.push(cid.gid);
        }
    }
    return v;
}

fn find_out_ss(id: u64, gid: u64) -> u64 {
    for cid in data_state.user_in_group {
        if cid.uid == id and cid.gid == gid {
            return santa_for;
        }
    }
}