use crate::io::files::filemode::Filemode;

use crate::sys::log::config::LoggingConfigData;
use crate::sys::log::level::LoggingLevel;
use crate::sys::time::LseTime;

pub struct LseLogging{
    config:     Option<LoggingConfigData>,
}

impl LseLogging {
    pub fn new(level:  u8, _filepath: String, _format: String,
        _datefmt: String, _ext_info: bool) -> LseLogging {

        let filemode : char = Filemode::Write.into_char();
        let (_level, level_name) : (u8, String) = LoggingLevel::into_tuples(level);

        LseLogging{
            config:   Some(LoggingConfigData::Basic{
                level:      _level,
                name:       level_name,
                filemode:   filemode,
                filepath:   _filepath,
                format:     _format,
                datefmt:    _datefmt,
                ext_info:   _ext_info
            })
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

    pub fn log(&self, _format: &str, _value_str : &Vec<&str> ,_config: Option<LoggingConfigData>) {

        let mut config : Option<&LoggingConfigData> = self.config.as_ref();

        match _config {
            Some(ref a) => {
                config = Some(a);
            },
            None => {}
        }

        let mut final_fmt: String = "".to_string();

        let (level_int, level_name) : (u8, String);

        match config {
            Some(ref level_config) => {
                match level_config {
                    LoggingConfigData::Basic {level, name, format, ..} | LoggingConfigData::Critical {format, level, name, ..} => {
                        final_fmt = format.to_owned();
                        (level_int, level_name) = (*level, name.to_owned());
                    }
                }
            },
            None => {
                (level_int, level_name) = (0, "[NOTSET]".to_string());
            }
        }

        let time = LseTime::now();

        match time {
            Ok(a) => {
                let mut time : String = "".to_string();
                time += &(a.year.as_str().to_owned() + "/" + a.month.as_str() + "/"+ a.day.as_str() + " ") ;
                time += &("[".to_string() + a.hour.as_str() + ":" + a.minutes.as_str() + "] -");
                final_fmt = final_fmt.replace("{time}", time.as_str());
            },
            Err(_) => {
                final_fmt = final_fmt.replace("{time}", "Not found -");
            }
        }

        final_fmt = final_fmt.replace("{level}", level_int.to_string().as_str());
        final_fmt = final_fmt.replace("{level_name}", level_name.as_str());
        final_fmt  = final_fmt.replace("{message}", LseLogging::change_fmt(_format, _value_str).as_str());

        println!("{}", final_fmt);

        if level_int == 16 {
            panic!("{} ", final_fmt);
        }
    }
}
