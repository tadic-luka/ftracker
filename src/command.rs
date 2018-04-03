extern crate serde;

use std::process;
use self::serde::{Deserialize, Deserializer};

use std::os::unix::process::CommandExt;
use std::io;

use utils::{
    get_current_username, 
    get_current_groupname,
    get_user_id,
    get_group_id
};
#[derive(Debug, Clone, Deserialize)]
pub struct Command {
    user: Option<String>,
    env: Option<Vec<(String, String)>>,
    #[serde(deserialize_with = "parse_command")]
    command: Vec<String>,
    working_dir: Option<String>,
    group: Option<String>
}


fn parse_command<'de, D>(deserializer: D)  -> Result<Vec<String>, D::Error>
where
D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let mut command: Vec<String> = Vec::new();
    let mut token = String::new();
    let mut in_quotes = false;
    for ch in s.chars() {
        match ch {
            '\"' => {
                if in_quotes {
                    command.push(token.clone());
                    token.clear();
                }
                in_quotes = !in_quotes;
            },
            c @ ' ' |  c @ '\t' | c @ '\n' => {
                if ! in_quotes && token.len() > 0 {
                    command.push(token.clone());
                    token.clear();
                } else if in_quotes  {
                    token.push(c);
                }
            },
            c @ _ => {
                token.push(c);
            }
        }
    }
    if token.len() > 0 {
        command.push(token.clone());
    }
    Ok(command)
}


impl Command {
    pub fn get_user_ref(&self) -> Option<&String> {
        self.user.as_ref()
    }
    pub fn get_group_ref(&self) -> Option<&String> {
        self.group.as_ref()
    }

    pub fn get_user(&self) -> String {
        self.user.clone()
            .unwrap_or(get_current_username().unwrap())
    }

    pub fn get_group(&self) -> String {
        self.group.clone()
            .unwrap_or(get_current_groupname().unwrap())
    }
    pub fn get_command(&self) -> String {
        self.command.join(" ")
    }
    pub fn execute(&self) -> io::Result<process::Output> {
        process::Command::new(&self.command[0])
            .args(self.command.iter().skip(1))
            .envs(self.env.clone().unwrap_or(vec![]).into_iter())
            .uid(get_user_id(self.user.as_ref()))
            .gid(get_group_id(self.group.as_ref()))
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::null())
            .output() 
    }

}
