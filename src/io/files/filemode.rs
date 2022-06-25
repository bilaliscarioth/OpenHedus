pub enum Filemode {
    Read,
    Write,
    Exec,
    All,
    None
}

impl Filemode {
    pub fn into_enum(filemode: char) -> Filemode {
        match filemode {
            'r' => { Filemode::Read },
            'w' => { Filemode::Write },
            'x' => { Filemode::Exec },
            'a' => { Filemode::All },
            _ => { Filemode::None }
        }
    }

    pub fn into_char(&self) -> char {
        match self {
            Filemode::Read =>  { 'r' },
            Filemode::Write => { 'w' },
            Filemode::Exec =>  { 'x' },
            Filemode::All =>   { 'a' },
            Filemode::None =>  { '\0' }
        }
    }
}
