extern crate config;

use config::{File, FileFormat};

pub struct Config {
    pub resource_location: String,
    pub ip: String,
    pub port: String,
    pub default_errors: bool,
}

impl Config {
    pub fn new() -> Config {
        // Getting the config file
        let mut config = config::Config::default();
        config.merge(File::with_name("insult")).unwrap().merge(config::Environment::with_prefix("APP")).unwrap();

        // Loading configuration
        let resource_location = match config.get_str("htdocs-root") {
            Ok(a) => a,
            Err(_) => {println!("BITCH ASS NIGGA www"); String::from("www")}
        };

        let ip = match config.get_str("server-address"){
            Ok(a) => a,
            Err(_) => {println!("FUCKING ASS WIPE 127.0.0.1"); String::from("127.0.0.1")}
        };

        let port = match config.get_str("server-port"){
            Ok(a) => a,
            Err(_) => {println!("FUCKING CUNT 8080"); String::from("8080")}
        };

        let default_errors = match config.get_bool("use-builtin-error-pages"){
            Ok(a) => a,
            Err(_) => {println!("FUCKING CUNT 8080"); true}
        };

        Config {
            resource_location,
            ip,
            port,
            default_errors
        }
    }
}
