std::fs::File;

pub struct PakEntry {
    pub filename:   [char; 56],
    pub offset:     u32,
    pub size:       u32
}

struct PakHeader {
    ident:      i32,
    dir_offset: u32,
    dir_size:   u32
}

pub struct PAK {
    header :        PakHeader,
    files :         [PakEntry],
    file :          File
}

impl PAK {
    pub fn open(filepath: String) -> Self {
        let  magic_header :  u32 = ('K'<<24)+('C'<<16)+('A'<<8)+'P';

        let file = File::open(filepath)?;

        Pak{
        }
    }

    pub fn get_files() -> PakEntry {

    }
}
