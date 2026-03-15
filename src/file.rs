pub struct File {
    name: String,
    contents: &'static str,
}

impl File {
    pub fn new(
        name: String,
        contents: &'static str,
    ) -> Self {
        Self { name, contents }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_contents(&self) -> &'static str {
        &self.contents
    }
}
