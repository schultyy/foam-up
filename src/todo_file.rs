
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Default)]
pub struct TodoFile { }

impl TodoFile {
    pub fn write_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let mut file = File::create(path.join("todo.md"))?;

        writeln!(&mut file, "# Todo\n")?;
        writeln!(&mut file, "You can create todos in Foam.\n")?;
        writeln!(&mut file, "- [x] This is an example of a todo list item that's complete.")?;
        writeln!(&mut file, "- [ ] This one is not complete yet.")?;
        Ok(())
    }
}