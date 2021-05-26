use std::fs::File;
use std::io::Write;
use std::path::Path;
use serde::{Serialize};

#[derive(Serialize)]
pub struct Extension {
    recommendations: Vec<String>
}

impl Extension {
    pub fn new() -> Self {
        let recommendations = vec!("foam.foam-vscode", "yzhang.markdown-all-in-one", "mushan.vscode-paste-image", "ban.spellright")
        .iter()
        .map(|s| s.to_string())
        .collect();

        Self {
            recommendations: recommendations
        }
    }

    pub fn write_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let json_string = serde_json::to_string_pretty(&self).unwrap();
        let mut file = File::create(path.join(".vscode").join("extensions.json"))?;
        writeln!(&mut file, "{}", json_string)?;
        Ok(())
    }
}