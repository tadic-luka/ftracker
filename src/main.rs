#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
extern crate rustc_serialize;
extern crate log4rs;
extern crate toml;
extern crate notify;
extern crate docopt;

mod errors;
mod rules;
mod command;
mod utils;
mod watcher;
mod thread_pool;

use watcher::FileWatcher;

use docopt::Docopt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
File Watcher

Usage:
    fwatcher [-c PATH] [options]

Options:
    -h --help       Shows this message.
    -v --version    Shows version.
    -l --log        Use logging.
    -c PATH         Set path to config file.
";

#[derive(Debug, RustcDecodable)]
pub struct Args {
    pub flag_c: Option<String>,
    pub flag_h: bool,
    pub flag_v: bool,
    pub flag_l: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.decode())
        .unwrap_or_else(|e| e.exit());

    let mut path = String::from(".watch.toml");

    match args {
        Args { flag_v: true, .. } => show(VERSION),
        Args { flag_h: true, .. } => show(USAGE),

        Args { flag_l: true, flag_c: p, .. } => {
            path = p.unwrap_or(path);
            log4rs::init_file("log4rs.yaml", log4rs::file::Deserializers::new()).unwrap();
        },
        Args { flag_c: Some(p), .. } => {
            path = p;
        },
        _ => {}

    }
    info!("booting up");
    match FileWatcher::from_file(&path, 2) {
        Ok(mut watcher) => watcher.run(),
        Err(err) => eprintln!("Could not start watcher, reason: {}", err)
    }
}

fn show(text: &str) -> ! {
    println!("{}", text);
    std::process::exit(0)
}
