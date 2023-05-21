use std::{path::PathBuf, fs};

pub struct Configuration {
    root_dir: String,
    file_endings: Vec<String>,
}

impl Configuration {
    fn new(root_dir: String, file_endings: Vec<String>) -> Configuration {
        Configuration { root_dir, file_endings }
    }
}

pub struct ConfigurationBuilder {
    root_dir: String,
    file_endings: Option<Vec<String>>,
}

impl ConfigurationBuilder {
    pub fn new(root_dir: String) -> Self {
        ConfigurationBuilder {root_dir: root_dir, file_endings: None}
    }

    pub fn add_file_endings(self, file_endings: Vec<String>) -> Self {
        ConfigurationBuilder { root_dir: self.root_dir, file_endings: Some(file_endings) }
    }

    pub fn build(self) -> Configuration {
        Configuration::new(self.root_dir, self.file_endings.unwrap_or(Vec::new()))
    }
}

type PathCollection = Vec<PathBuf>;

pub fn get_files_in_directory(config: &Configuration) -> PathCollection {
    let mut initial_files = fs::read_dir(&config.root_dir)
        .unwrap()
        .map(|result| result.unwrap())
        .collect::<Vec<_>>();

    let mut files = PathCollection::new();

    let mut index = 0;
    while index < initial_files.len() {
        let file = &initial_files[index];
        if file.path().is_dir() {
            let dir_entries = fs::read_dir(file.path())
                .unwrap()
                .map(|result| result.unwrap())
                .collect::<Vec<_>>();
            initial_files.extend(dir_entries);
        } else {
            if config.file_endings.contains(&file.path().extension().unwrap().to_string_lossy().to_string()) {
                continue;
            }
            files.push(file.path());
        }

        index += 1;
    }

    files
}