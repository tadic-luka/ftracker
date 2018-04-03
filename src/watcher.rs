extern crate log4rs;
extern crate toml;
extern crate notify;

use self::notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent, watcher};
use std::sync::mpsc;
use std::sync::Arc;
use std::io::Read;
use std::fs::File;



use std::time::Duration;

use thread_pool::ThreadPool;
use rules::Rule;
use errors::Errors;

pub struct FileWatcher {
    rules: Arc<Vec<Rule>>,
    rx: mpsc::Receiver<DebouncedEvent>,
    _watcher: RecommendedWatcher,
    pool: ThreadPool,
}

impl FileWatcher {
    pub fn from_file(filename: &str, cores: usize) -> Result<Self, Errors> {
            let mut filename = String::from(filename);
            let mut file = File::open(&filename)?;
            filename.clear();
            file.read_to_string(&mut filename)?;
            FileWatcher::init(&filename, cores)
    }

    pub fn init(config: &str, cores: usize) -> Result<Self, Errors> {
        let mut rules = Rule::rules_from_str(&config)?;

        let (tx, rx) = mpsc::channel();
        let mut watch =  watcher(tx, Duration::from_millis(100))?;
        FileWatcher::start_watching(&mut rules, &mut watch);
        
        if rules.len() == 0 {
            return Err(Errors::NoRules);
        }
        return Ok(FileWatcher {
            rules: Arc::new(rules),
            rx: rx,
            _watcher: watch,
            pool: ThreadPool::new(cores)

        });
    }
    fn start_watching(rules: &mut Vec<Rule>, watch: &mut RecommendedWatcher) {
        rules.retain(|rule| {
            if let Err(e) = watch.watch(rule.get_dir(), RecursiveMode::NonRecursive) {
                error!("Failed to watch dir {} for rule '{}', reason: {}, removing rule ...", rule.get_dir(), rule.get_name(), e);
                return false;
            }
            return true;
        });
    }

    pub fn run(&mut self) {
        for event in self.rx.iter() {
            match event {
                DebouncedEvent::Write(pb) |
                    DebouncedEvent::Create(pb) |
                    DebouncedEvent::Chmod(pb) => {
                        let rules = Arc::clone(&self.rules);
                        self.pool.execute(move || {
                            for rule in rules.iter() {
                                if rule.watches_path(pb.to_str()) {
                                    rule.execute_command();
                                }
                            }
                        });
                    },
                _ => {}
            }
        }

    }
}

