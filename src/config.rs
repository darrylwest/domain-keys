const DFLT_ROUTES: u8 = 1;
const DFLT_KEY_SIZE: u8 = 16;

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub routes: u8,
    pub key_size: u8,
}

impl Config {
    pub fn new() -> Config {
        Config {
            routes: DFLT_ROUTES,
            key_size: DFLT_KEY_SIZE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let config = Config::new();

        assert_eq!(config.routes, DFLT_ROUTES);
        assert_eq!(config.key_size, DFLT_KEY_SIZE);
    }

    #[test]
    fn update() {
        let mut config = Config::new();

        config.routes = 24;
        config.key_size = 8;

        assert_eq!(config.routes, 24);
        assert_eq!(config.key_size, 8);
    }
}
