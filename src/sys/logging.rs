use std::time::{SystemTime};

pub enum LoggingLevel{
    NOTSET,
    DEBUG,
    INFO,
    WARNING,
    ERROR,
    CRITICAL
}

pub enum LoggingConfigData {
    Basic{
        filepath:   String,
        filemode:   char,
        level:      u8,
        format:     String,
        datefmt:    String,
        ext_info:   bool,
        name:       String
    },

    Critical{
        filepath:   String,
        filemode:   char,
        level:      u8,
        format:     String,
        datefmt:    String,
        ext_info:   bool,
        name:       String,
        throw_error:    String
    }
}

impl LoggingConfigData{
    pub fn new(_level:  LoggingLevel, filemode: char, filepath: String, format: String, 
        datefmt: String, ext_info: bool) -> Self {

        let mut level : u8 = 0;
        let mut level_name : String = "".to_string();
        match _level {
            LoggingLevel::NOTSET => {
                level = 0;
                level_name = "[NOTSET]".to_string();
            },
            LoggingLevel::DEBUG => {
                level = 1;
                level_name = "[DEBUG]".to_string();
            },
            LoggingLevel::INFO => {
                level = 2;
                level_name = "[INFO]".to_string();
            },
            LoggingLevel::WARNING => {
                level = 4;
                level_name = "[WARNING]".to_string();
            },
            LoggingLevel::ERROR => {
                level = 8;
                level_name = "[ERROR]".to_string();
            },
            LoggingLevel::CRITICAL => {
                level = 16;
                level_name = "[CRITICAL]".to_string();
            }
        }

        LoggingConfigData::Basic{
                    level:      level,
                    name:       level_name,
                    filemode:   filemode,
                    filepath:   filepath,
                    format:     format,
                    datefmt:    datefmt,
                    ext_info:   ext_info
        }
    }

    pub fn change_level(&self, _level :LoggingLevel) -> Option<Self> {
        match self {
            Self::Basic{name, filemode, filepath, format, datefmt, ext_info, ..} |
            Self::Critical{name, filemode, filepath, format, datefmt, ext_info, ..} => {
                Some(LoggingConfigData::new(
                    _level,
                    *filemode,
                    filepath.to_string(),
                    format.to_string(),
                    datefmt.to_string(),
                    *ext_info
                ))
            }

            _ => {None}
        }
    }

    pub fn get(&self, _config: Option<LoggingConfigData>) -> Option<Self> {
        match _config.unwrap() {
            LoggingConfigData::Basic{filepath, filemode, level, format, datefmt, ext_info, name} => {
                Some(LoggingConfigData::Basic{
                    filepath:   filepath,
                    filemode:   filemode,
                    level:      level,
                    format:     format,
                    datefmt:    datefmt,
                    ext_info:   ext_info,
                    name:       name
                })
            }
            _ => {None}
        }
    }

    pub fn get_level_info(&self) -> (u8, String) {
        match self{
            Self::Basic{level, name, ..} => {(*level, name.to_string())},
            Self::Critical{level, name, ..} => {(*level, name.to_string())}
        }
    }
}

pub struct LseLogging{
    config:     Option<LoggingConfigData>,
}

impl LseLogging {
    pub fn new(_config: Option<LoggingConfigData>) -> LseLogging {
        LseLogging{
            config:     _config
        }
    }

    fn change_fmt(_format: &str, _value_str: &Vec<&str>) -> String{
        let mut result : String = _format.to_string();
        for i in 0.._value_str.len() {
            let index : String  = "{".to_string() + &i.to_string() + &"}".to_string();
            result = result.replace(index.as_str(), _value_str[i]);
        }
        result
    }

    pub fn log(&self, _format: &str, _value_str : &Vec<&str> ,_config: Option<LoggingConfigData>) -> String {
        /*
         * if coders want to change config
         */

        match _config {
            Some(_) => {
            },
            None => {}
        }
        let time = SystemTime::now().elapsed().unwrap().as_secs();
        let (level, level_name) = self.config.as_ref().unwrap().get_level_info();
        let final_fmt : String = LseLogging::change_fmt(_format, _value_str);

        println!("{}, {} {}", time, level_name,final_fmt);

        final_fmt
    }
}
