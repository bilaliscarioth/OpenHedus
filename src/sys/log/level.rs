pub enum LoggingLevel{
    NOTSET,
    DEBUG,
    INFO,
    WARNING,
    ERROR,
    CRITICAL
}

impl LoggingLevel {
    pub fn get(&self) -> (u8, String) {
        match self {
            LoggingLevel::DEBUG => { (1, "[DEBUG]".to_string())  },
            LoggingLevel::INFO => { (2, "[INFO]".to_string()) },
            LoggingLevel::WARNING => { (4, " [WARNING]".to_string()) },
            LoggingLevel::ERROR => { (8, "[ERROR]".to_string()) },
            LoggingLevel::CRITICAL => { (16, "[CRITICAL]".to_string()) },
            LoggingLevel::NOTSET => { (0, "[NOTSET]".to_string()) }
        }
    }
    pub fn get_names(_level: u8) -> String  {
        match _level {
            1 => { "[DEBUG]".to_string()  },
            2 => { "[INFO]".to_string() },
            4 => { " [WARNING]".to_string() },
            8 => { "[ERROR]".to_string() },
            16 => { "[CRITICAL]".to_string() },
            _ => { "[NOTSET]".to_string() }
        }
    }
    pub fn into_tuples(_level: u8) -> (u8, String)  {
        match _level {
            1 => { (1, "[DEBUG]".to_string())  },
            2 => { (2, "[INFO]".to_string()) },
            4 => { (4, " [WARNING]".to_string()) },
            8 => { (8, "[ERROR]".to_string()) },
            16 => { (16, "[CRITICAL]".to_string()) },
            _ => { (0, "[NOTSET]".to_string()) }
        }
    }
}
