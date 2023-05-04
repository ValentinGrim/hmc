use std::env;
use std::path;
use ini::Ini;

// Const declaration
const CONF_NAME:      &str = "hmc.ini";
const SQLITE_DEFAULT: &str = "hmc.db";

// Error
pub enum Error
{
    IniError(ini::Error),
    IoError(std::io::Error),
    AlreadyInit,
}
//Struct declaration
pub struct Config 
{
    db_type: String,
    db_path: String, // For SQLite3
    is_init: bool,
}

impl Config 
{
    fn get_path(file: String) -> String
    {
        let mut dir = env::current_exe().unwrap();
        dir.pop();
        dir.push(file);
        dir.into_os_string().into_string().unwrap()
    }

    // Default configuration, should be created with this
    pub fn default() -> Config 
    {
        Config 
        {
            db_type: String::from("sqlite3"),
            db_path: Config::get_path(String::from(SQLITE_DEFAULT)),
            is_init: false,
        }
    }

    // Init the configuration struct, should 
    pub fn init(&mut self) -> Result<bool, Error> 
    {
        if self.is_init 
        {
            return Err(Error::AlreadyInit);
        }

        // Config doesn't exist
        if !(path::Path::new(&Config::get_path(String::from(CONF_NAME))).exists()) 
        {
            // Generate default config file
            let mut conf = Ini::new();
            conf.with_section(Some("Database"))
                .set("db_type", &self.db_type)
                .set("db_path", &self.db_path);
            
            match conf.write_to_file(String::from(
                {
                    let mut dir = env::current_exe().unwrap();
                    dir.pop();
                    dir.push(CONF_NAME);
                    dir.into_os_string().into_string().unwrap() 
                }))
            {
                Ok(_) => (),
                Err(e) => return Err(Error::IoError(e))
            };

            self.is_init = true;
        }
        // Config exist, load it
        else 
        {
            let conf = Ini::load_from_file(Config::get_path(String::from(CONF_NAME)));
            match conf // Error handling
            {
                Ok(c) => c,
                Err(e) => return Err(Error::IniError(e)),
            };

        }
        Ok(self.is_init)
    }
}
