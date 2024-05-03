use std::error::Error;

pub struct App {}

struct AppList {}

impl Iterator for AppList {
    type Item = App;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl AppList {
    fn new() -> Result<Self, Box<dyn Error>> {
        Ok(AppList {})
    }
}

impl App {
    pub fn list() -> Result<impl Iterator<Item = Self>, Box<dyn std::error::Error>> {
        Ok(AppList::new().ok().into_iter().flatten())
    }
}
