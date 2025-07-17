pub struct Engine {
    title: String
}

impl Engine {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string()
        }
    }
}

