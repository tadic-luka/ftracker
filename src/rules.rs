extern crate toml;
extern crate glob;
extern crate serde;

use command::Command;
use utils::{path_exists, user_exists, group_exists};
use errors::Errors;
use self::serde::{Deserialize, Deserializer};
use self::glob::{Pattern, MatchOptions};

type MatchPatterns = Vec<Pattern>;

const MATCH_OPTS: MatchOptions = MatchOptions {
    case_sensitive: true,
    require_literal_separator: true,
    require_literal_leading_dot: false
};




#[derive(Debug, Clone, Deserialize)]
pub struct Rule {
    name: String,
    dir: String,
    command: Command,
    #[serde(deserialize_with = "make_glob_pattern")]
    watch_patterns: MatchPatterns,
    #[serde(default = "default_recursive")]
    recursive: bool
}

#[derive(Debug, Clone, Deserialize)]
struct Rules {
    rules: Vec<Rule>
}

fn default_recursive() -> bool {
    false
}

fn make_glob_pattern<'de, D>(deserializer: D)  -> Result<MatchPatterns, D::Error>
where
D: Deserializer<'de>,
{
    let s: Vec<&str> = Deserialize::deserialize(deserializer)?;
    Ok(
        s.into_iter()
        .map(|value| Pattern::new(value).unwrap())
        .collect()
      )
}
impl Rule {

    pub fn rules_from_str(config: &str) ->  Result<Vec<Rule>, Errors> {
        let mut r: Rules = toml::from_str(config)?;
        Self::check_rules(&mut r.rules);
        if r.rules.len() == 0 {
            return Err(Errors::NoRules);
        }
        Ok(r.rules)
    }

    fn check_rules(rules: &mut Vec<Rule>) {
        rules.retain(|rule| {
            if !path_exists(&rule.dir) {
                error!("For rule '{}', path {} does not exist, removing rule...", rule.name, rule.dir);
                return false;
            }
            if !user_exists(rule.command.get_user_ref()) {
                error!("For rule '{}', user {} does not exist, removing rule...", rule.name, rule.command.get_user_ref().unwrap());
                return false;
            }
            if !group_exists(rule.command.get_group_ref()) {
                error!("For rule '{}', user {} does not exist, removing rule...", rule.name, rule.command.get_group_ref().unwrap());
            }
            return true;
        });
    }
    pub fn execute_command(&self) {
        match self.command.execute() {
            Ok(output) => {
                if output.stderr.len() == 0 {
                    info!("Executed command '{}' for rule '{}' as user {} and group {}", self.command.get_command(), self.name, self.command.get_user(), self.command.get_group());
                } else {
                    error!("Error while executing command '{}' for rule '{}' as user {} and group {}, output is: {}", self.command.get_command(), self.name, self.command.get_user(), self.command.get_group(), String::from_utf8_lossy(&output.stderr[0..output.stderr.len()-1]));

                }
            }
            Err(e) => {
                error!("Could not execute command '{}' for rule '{}', reason: {}", self.command.get_command(), self.name, e);
            }
        }
    }


    fn matches_dir(&self, dir: Option<&str>) -> bool {
        match dir {
            Some(d) => d.starts_with(&self.dir),
            None => false
        }
    }
    pub fn watches_path(&self, path: Option<&str>) -> bool {
        if ! self.matches_dir(path) {
            return false;
        }
        match path {
            Some(p) => self.watch_patterns.iter().any(|pattern| pattern.matches_with(p, &MATCH_OPTS)),
            None => false
        }

    }

    pub fn get_dir(&self) -> &str {
        &self.dir
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    extern crate toml;
    use super::Rule;
    use super::Rules;
    use super::RuleError;
    use command::Command;

    #[test]
    fn rule_is_ok() {
        let rules = Rule::rules_from_str(r#"
        [[rules]]
        name = 'desetak'
        dir = '/home/luka'
        watch_patterns = ['*.toml', 'Makefile']
        [rules.command]
        command = 'echo uspjeh'
        user = 'luka'
        [[rules]]
        name = 'pedesetak'
        dir = '/home/luka/alabaster'
        watch_patterns = ['*.toml', 'Makefile']
        [rules.command]
        command = 'echo uspjeh'
        "#).unwrap();
        println!("rules su:");
        for rule in rules {
            println!("{:?}", rule);
        }
    }
}
