use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::fs;
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone)]
struct FileContent {
    content: String,
    last_modified: std::time::SystemTime,
}

struct ContentManager {
    content: Arc<RwLock<HashMap<String, FileContent>>>,
}

impl ContentManager {
    fn new() -> Self {
        ContentManager {
            content: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn load_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)?;
        let metadata = fs::metadata(path)?;

        let mut content_map = self.content.write().unwrap();
        content_map.insert(
            path.to_string_lossy().into_owned(),
            FileContent {
                content,
                last_modified: metadata.modified()?,
            }
        );
        Ok(())
    }

    fn search(&self, path: &str, pattern: &str) -> Vec<(usize, String)> {
        let content_map = self.content.read().unwrap();

        if let Some(file_content) = content_map.get(path) {
            let regex = Regex::new(pattern).unwrap_or_else(|_| Regex::new(&regex::escape(pattern)).unwrap());

            file_content.content
                .lines()
                .enumerate()
                .filter(|(_, line)| regex.is_match(line))
                .map(|(num, line)| (num + 1, line.to_string()))
                .collect()
        } else {
            Vec::new()
        }
    }

    fn update_if_changed<P: AsRef<Path>>(&self, path: P) -> std::io::Result<bool> {
        let path = path.as_ref();
        let metadata = fs::metadata(path)?;
        let current_modified = metadata.modified()?;

        let mut needs_update = false;

        if let Some(content) = self.content.read().unwrap().get(path.to_string_lossy().as_ref()) {
            if current_modified != content.last_modified {
                needs_update = true;
            }
        } else {
            needs_update = true;
        }

        if needs_update {
            self.load_file(path)?;
        }

        Ok(needs_update)
    }
}
