use std::fs;
use std::path::Path;

use crate::inbox_file::InboxFile;
use crate::todo_file::TodoFile;
use crate::vscode_templates;

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

        let templates_dir = Path::new(&self.path).join(".foam").join("templates");
        fs::create_dir_all(templates_dir)?;
        Ok(())
    }

    pub fn create_files(&self) -> Result<(), std::io::Error> {
        let todo_file = TodoFile::default();
        todo_file.write_file(Path::new(&self.path))?;
        let inbox_file = InboxFile::default();
        inbox_file.write_file(Path::new(&self.path))?;
        let extensions_file = vscode_templates::Extension::new();
        extensions_file.write_file(Path::new(&self.path))?;
        let foam_file = vscode_templates::Foam::new();
        foam_file.write_file(Path::new(&self.path))?;
        let settings_file = vscode_templates::Settings::new();
        settings_file.write_file(Path::new(&self.path))?;
        let blog_post_template = vscode_templates::BlogPostTemplate::default();
        blog_post_template.write_file(Path::new(&self.path))?;
        Ok(())
    }
}