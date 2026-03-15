pub struct File<'a> {
    name: &'a str,
    contents: String,
}

impl<'a> File<'a> {
    pub fn new(
        name: &'a str,
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
