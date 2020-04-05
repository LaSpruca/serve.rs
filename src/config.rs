extern crate config;

use config::{File, FileFormat};

pub struct Config {
    pub resource_location: String,
    pub ip: String,
    pub port: String
}

impl Config {
    pub fn new() -> Config {
        let mut config = config::Config::default();
        config.merge(File::with_name("insult")).unwrap().merge(config::Environment::with_prefix("APP")).unwrap();
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

        Config {
            resource_location,
            ip,
            port
        }
    }
}
