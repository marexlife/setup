pub struct FileData {
    name: String,
    contents: String,
}

impl FileData {
    pub fn new(
        name: String,
        contents: String,
    ) -> Self {
        Self { name, contents }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_contents(&self) -> &str {
        &self.contents
    }
}
