use std::fs::File;
use std::io::Write;
use std::path::Path;
use serde::{Serialize};
use serde_json::Map;
use serde_json::Number;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default)]
pub struct BlogPostTemplate {

}

impl BlogPostTemplate {
    pub fn write_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let mut file = File::create(path.join(".foam").join("templates").join("blogpost.md"))?;
        writeln!(&mut file, "{}", "---")?;
        writeln!(&mut file, "{}", "date: ${CURRENT_MONTH}/${CURRENT_DATE}/${CURRENT_YEAR}")?;
        writeln!(&mut file, "{}", "Categories: null")?;
        writeln!(&mut file, "{}", "Keywords:")?;
        writeln!(&mut file, "{}", "Research:")?;
        writeln!(&mut file, "{}", "seo_title:")?;
        writeln!(&mut file, "{}", "slug:")?;
        writeln!(&mut file, "{}", "excerpt: null")?;
        writeln!(&mut file, "{}", "seo_description: null")?;
        writeln!(&mut file, "{}", "---")?;
        writeln!(&mut file, "{}", "# ${TM_FILENAME_BASE}")?;

        Ok(())
    }
}

#[derive(Serialize)]
pub struct Settings {
    pairs: HashMap<String, Value>
}

impl Settings {
    pub fn new() -> Self {
        let mut hash_map = HashMap::new();
        hash_map.insert("editor.minimap.enabled".to_string(), Value::Bool(false));
        hash_map.insert("editor.wrappingIndent".to_string(), Value::String("Indent".to_string()));
        hash_map.insert("editor.overviewRulerBorder".to_string(), Value::Bool(false));
        hash_map.insert("editor.lineHeight".to_string(), Value::Number(Number::from(24)));
        hash_map.insert( "editor.fontFamily".to_string(), Value::String("Cascadia Mono".to_string()));

        let mut markdown_map = Map::new();
        markdown_map.insert("editor.QuickSuggestions".to_string(), Value::Bool(true));
        hash_map.insert("[markdown]".to_string(), Value::Object(markdown_map));

        hash_map.insert("files.defaultLanguage".to_string(), Value::String("markdown".to_string()));

        let mut files_exclude = Map::new();
        files_exclude.insert("**/node_modules".to_string(), Value::Bool(true));
        hash_map.insert("files.exclude".to_string(), Value::Object(files_exclude));

        let mut watcher_exclude = Map::new();
        watcher_exclude.insert("**/node_modules".to_string(), Value::Bool(true));
        hash_map.insert("files.watcherExclude".to_string(), Value::Object(watcher_exclude));


        hash_map.insert("foam.edit.linkReferenceDefinitions".to_string(), Value::String("withExtensions".to_string()));
        hash_map.insert("foam.openDailyNote.directory".to_string(), Value::String("journal".to_string()));
        hash_map.insert("foam.openDailyNote.titleFormat".to_string(), Value::String("fullDate".to_string()));
        hash_map.insert("git.enableSmartCommit".to_string(), Value::Bool(true));
        hash_map.insert("git.postCommitCommand".to_string(), Value::String("sync".to_string()));
        hash_map.insert("markdown.preview.breaks".to_string(), Value::Bool(true));
        hash_map.insert("pasteImage.path".to_string(), Value::String("${projectRoot}/attachments".to_string()));
        hash_map.insert("pasteImage.showFilePathConfirmInputBox".to_string(), Value::Bool(true));
        hash_map.insert("prettier.singleQuote".to_string(), Value::Bool(false));
        hash_map.insert("spellright.notificationClass".to_string(), Value::String("warning".to_string()));
        hash_map.insert("vscodeMarkdownNotes.noteCompletionConvention".to_string(), Value::String("noExtension".to_string()));
        hash_map.insert("vscodeMarkdownNotes.slugifyMethod".to_string(), Value::String("github-slugger".to_string()));
        hash_map.insert("editor.rulers".to_string(), Value::Array([].to_vec()));
        hash_map.insert("editor.fontSize".to_string(), Value::Number(Number::from(20)));

        Self {
            pairs: hash_map
        }
    }

    pub fn write_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let json_string = serde_json::to_string_pretty(&self.pairs).unwrap();
        let mut file = File::create(path.join(".vscode").join("settings.json"))?;
        writeln!(&mut file, "{}", json_string)?;
        Ok(())
    }
}

#[derive(Serialize)]
pub struct Foam {
    purpose: String,
    future: String
}

impl Foam {
    pub fn new() -> Self {
        Self {
            purpose: "this file exists to tell the foam-vscode plugin that it's currently in a foam workspace".to_string(),
            future: "we may use this for custom configuration".to_string()
        }
    }

    pub fn write_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let json_string = serde_json::to_string_pretty(&self).unwrap();
        let mut file = File::create(path.join(".vscode").join("foam.json"))?;
        writeln!(&mut file, "{}", json_string)?;
        Ok(())
    }
}

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

