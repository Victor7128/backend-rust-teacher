use std::env;

pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = env::var("DATABASE_URL")?;
        // Puedes setear un puerto por defecto si no existe la variable
        let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string()).parse()?;
        Ok(Self {
            database_url,
            port,
        })
    }
}