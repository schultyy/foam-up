use std::fs;

pub struct Project {
    path: String
}

impl Project {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string()
        }
    }

    pub fn create(&self) -> Result<(), std::io::Error> {
        fs::create_dir_all(self.path.clone())?;
        Ok(())
    }
}