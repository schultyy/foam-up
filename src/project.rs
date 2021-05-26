use std::fs;
use std::path::Path;

use crate::todo_file;
use crate::todo_file::TodoFile;

pub struct Project {
    path: String
}

impl Project {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string()
        }
    }

    pub fn create_directories(&self) -> Result<(), std::io::Error> {
        fs::create_dir_all(self.path.clone())?;

        let vscode_dir = Path::new(&self.path).join(".vscode");
        fs::create_dir_all(vscode_dir)?;

        let templates_dir = Path::new(&self.path).join(".foam").join(".templates");
        fs::create_dir_all(templates_dir)?;
        Ok(())
    }

    pub fn create_files(&self) -> Result<(), std::io::Error> {
        let todo_file = TodoFile::default();
        todo_file.write_file(Path::new(&self.path))?;
        Ok(())
    }
}