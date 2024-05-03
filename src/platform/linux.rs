pub struct App {}

impl App {
    pub fn list() -> Result<impl Iterator<Item = Self>, Box<dyn std::error::Error>> {
        Vec::new()
    }
}
