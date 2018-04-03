# File watcher

Lightweight file watcher.
Listens for file changes specified in config file and then executing commands also specified in config file.
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
] # in form of ['key', 'value'], not required

# another rule which will exectue commands as user who executed this program
[[rules]]
name = 'some other rule'
dir = '/path/to/dir'
watch_patterns = ['**/filename']
[rules.command]
command = '/path/to/command'
```
This will create two rules, one will execute commands as specidied user, group and working dir, other will execute commands as user and group who executed watcher and in current working dir.
### Features
Ftracker watches on file create, file write and changing file mode bits.
Filename matching uses glob patterns.


#### Logging
Ftracker uses [log4rs](https://github.com/sfackler/log4rs) as logging framework.

#### Users and groups
Ftracker can execute command for given rule as user and/or group specified in config file.

#### Environment vars
Adding environment variables for command.


## Motivation
Learn Rust and create somewhat useful program.

## Installing
Easiest way to install this program is with [cargo](https://doc.rust-lang.org/cargo/).
```bash
# debug version
cargo build 
# release version
cargo build --release
# to execute
cargo run # --release
# or you can execute binary file directly
./target/debug/ftracker # debug version
./target/release/ftracker # release version
```

## License

Ftracker is licensed under 

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

## ROADMAP
   * [x] specify user for command
   * [x] specify group for command
   * [x] support logging
   * [ ] support live reload of config file
   * [ ] specify working dir for command
