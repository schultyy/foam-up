
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Default)]
pub struct InboxFile { }

impl InboxFile {
    pub fn write_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let mut file = File::create(path.join("inbox.md"))?;

        writeln!(&mut file, "# Inbox\n")?;
        writeln!(&mut file, "- Here you can write disorganized notes to be categorized later.")?;
        writeln!(&mut file, "- Bullet Points are useful but it could be free form text as well.")?;
        Ok(())
    }
}