/// Structs to deserialize the env.toml files
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub scheme: String,
    pub host: String,
    pub port: u16,
    pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl DatabaseConfig {
    pub fn to_url(&self) -> Url {
        assert!(
            matches!(self.scheme.as_str(), "postgres" | "postgresql"),
            "database.scheme must be postgres or postgresql"
        );

        let mut url = Url::parse(&format!("{}://{}", self.scheme, self.host))
            .expect("invalid scheme or host");

        url.set_port(Some(self.port))
            .expect("invalid port");

        url.set_username(&self.username)
            .expect("invalid username");

        url.set_password(Some(&self.password))
            .expect("invalid password");

        url.set_path(&self.name);

        url
    }
}


