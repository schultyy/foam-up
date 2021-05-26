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
}