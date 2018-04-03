# File watcher

Lightweight file watcher.
Configure execution of multiple rules specified in toml file.
Supports executing arbitrary amount of different rules with different users.
It is intended to run as root if users are specified for executing commands.


Example of toml file:
```toml
# .watch.toml
# list here all events and commands that it should execute

[[rules]]
name = 'run live preview on file change'
dir = '/path/to/dir'
watch_patterns = ['**/filename']
recursive = true # not required
[rules.command]
command = '/path/to/command'
user = 'user' # not required
group = 'group' # not required
working_dir = '/path/to/cwd/' # not required
env = [
	['MY_ENV', 'value']
] # not required

# another rule which will exectue commands as user who executed this program
[[rules]]
name = 'some other rule'
dir = '/path/to/dir'
watch_patterns = ['**/filename']
[rules.command]
command = '/path/to/command'
```
This will create two rules, one will execute commands as specidied user, group and working dir, other will execute commands as user and group who executed watcher and in current working dir.
Filename matching uses glob patterns.

## Motivation
Learn Rust and create somewhat useful program.

## Installing
Easiest way to install this program is with cargo.
```bash
# debug version
cargo build 
# release version
cargo build --release
# to execute
cargo run # --release
# or you can execute binary file directly
./target/debug/ftracker # debug version
./target/release/ftracker # debug version
```

## License

Ftracker is licensed under 

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

## Features
   * [x] specify user for command
   * [x] specify group for command
   * [x] support logging
   * [ ] support live reload of config file
   * [ ] specify working dir for command
