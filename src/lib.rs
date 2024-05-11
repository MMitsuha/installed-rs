mod platform;
pub use platform::App;

// passthru static
#[inline(always)]
pub fn list() -> Result<impl Iterator<Item = App>, Box<dyn std::error::Error>> {
    App::list()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_apps() {
        match list() {
            Ok(l) => {
                let _unused = l
                    .filter(|app| app.name().to_string().is_empty() == false)
                    .map(|app| {
                        let _name = app.name().to_string();
                        let _publisher = app.publisher().to_string();
                        let _version = app.version().to_string();
                    });
            }
            Err(_) => panic!(),
        };
    }
}
