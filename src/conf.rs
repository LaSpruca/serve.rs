extern crate config;

use config::File;
use std::ffi::OsString;
use std::io::Write;

pub struct Config {
    pub resource_location: String,
    pub ip: String,
    pub port: String,
    pub default_errors: bool,
}

impl Config {
    pub fn new() -> Config {
        let cwd = std::env::current_dir().unwrap();
        let cwd2 = OsString::from(cwd);
        let path = cwd2.to_str().unwrap();

        if !std::fs::metadata(&format!("{}/config.json", path)).is_ok() {
            let mut file = std::fs::File::create(&format!("{}/config.json", path)).unwrap();
            match file.write(b"{\n}\n"){
                Err(_) => println!("Error creating configuration file config.json"),
                Ok(_) => {}
            }
        }

        // Getting the config file
        let mut config = config::Config::default();
        match config.merge(File::with_name("config")) {
            Err(_) => println!("Error reading configuration file config.json"),
            Ok(_) => {}
        }

        match config.merge(config::Environment::with_prefix("APP")) {
            Err(_) => println!("Error reading configuration file config.json"),
            Ok(_) => {}
        }

        // Loading configuration
        let resource_location = match config.get_str("htdocs-root") {
            Ok(a) => a,
            Err(_) => String::from(".")
        };

        let ip = match config.get_str("server-address"){
            Ok(a) => a,
            Err(_) => String::from("127.0.0.1")
        };

        let port = match config.get_str("server-port"){
            Ok(a) => a,
            Err(_) => String::from("8080")
        };

        let default_errors = match config.get_bool("use-builtin-error-pages"){
            Ok(a) => a,
            Err(_) => true
        };

        Config {
            resource_location,
            ip,
            port,
            default_errors,
        }
    }
}
