use crate::globals::*;
use ini::Ini;
use std::path::PathBuf;
use std::{env, fs};

const CONF_FILE: &str = "hmc.ini";
const DEFAULT_DB: &str = "hmc.db";

pub struct HmcConfig {
    db_path: String,
    log_level: LogLevel,
}

impl HmcConfig {
    pub fn load() -> HmcConfig {
        /* Reconstruct config path */
        let mut config_path = env::current_exe().unwrap();
        config_path.pop();
        config_path.push(CONF_FILE);

        if fs::metadata(config_path.clone()).is_ok() {
            HmcConfig::init(config_path)
        } else {
            HmcConfig::new()
        }
    }

    fn new() -> HmcConfig {
        let mut config_dir = env::current_exe().unwrap();
        config_dir.pop();

        let mut db = config_dir.clone();
        db.push(DEFAULT_DB);

        let mut conf_file = config_dir.clone();
        conf_file.push(CONF_FILE);

        let mut conf = Ini::new();
        conf.with_section(Some("hmc"))
            .set("db_path", db.to_string_lossy())
            .set("log_level", LogLevel::WARN.to_string());
        conf.write_to_file(String::from(conf_file.to_string_lossy())).unwrap();

        HmcConfig::init(conf_file)
    }

    fn init(path: PathBuf) -> HmcConfig {
        HmcConfig {
            db_path: DEFAULT_DB.to_string(),
            log_level: LogLevel::WARN,
        }
    }
}
