use crate::io::files::filemode::Filemode;
use crate::sys::log::level::LoggingLevel;

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

///LoggingConfigData
impl LoggingConfigData{
    //!new
    /// Return a new config for logging system
    ///
    /// `` Arguments
    ///
    /// `` Returns
    ///
    ///
    pub fn new(_level:  LoggingLevel, _filepath: String, _format: String,
        _datefmt: String, _ext_info: bool) -> Self {

        let (level, level_name) = _level.get();
        let filemode : char = Filemode::Write.into_char();

        LoggingConfigData::Basic{
            level:      level,
            name:       level_name,
            filemode:   filemode,
            filepath:   _filepath,
            format:     _format,
            datefmt:    _datefmt,
            ext_info:   _ext_info
        }
    }

    pub fn change(&self, _level :LoggingLevel) -> Option<Self> {
        match self {
            Self::Basic{filepath, format, datefmt, ext_info, ..} |
            Self::Critical{filepath, format, datefmt, ext_info, ..} => {
                Some(LoggingConfigData::new(
                    _level,
                    filepath.to_string(),
                    format.to_string(),
                    datefmt.to_string(),
                    *ext_info
                ))
            },
            _ => {None}
        }
    }
}
