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
                let _ = l
                    .filter(|app| app.name().to_string().is_empty() == false)
                    .map(|app| {
                        let name = app.name().to_string();
                        let publisher = app.publisher().to_string();
                        let version = app.version().to_string();
                        println!("{} {} {}", name, publisher, version);
                    });
            }
            Err(_) => panic!(),
        };
    }
}
