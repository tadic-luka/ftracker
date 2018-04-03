extern crate users;
use std::path::Path;


pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn user_exists(user: Option<&String>) -> bool {
    user.map_or(true, |u| users::get_user_by_name(u).map(|_| true).unwrap_or(false))
}

pub fn group_exists(group: Option<&String>) -> bool {
    group.map_or(true, |g| users::get_group_by_name(g).map(|_| true).unwrap_or(false))
}

pub fn get_current_username() -> Option<String> {
    users::get_current_username()
}
pub fn get_current_groupname() -> Option<String> {
    users::get_current_groupname()
}

pub fn get_user_id(user: Option<&String>) -> u32 {
    user.and_then(|us|
                  users::get_user_by_name(us))
        .map(|us| us.uid())
        .unwrap_or(get_current_uid())
}
pub fn get_current_uid() -> u32 {
    users::get_current_uid()
}

pub fn get_group_id(group: Option<&String>) -> u32 {
    group.and_then(|gr|
                  users::get_group_by_name(gr))
        .map(|gr| gr.gid())
        .unwrap_or(get_user_id(None))
}
