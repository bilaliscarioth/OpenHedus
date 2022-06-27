use std::fs::File;
use std::io::{SeekFrom, Seek};
use std::io::{Error, ErrorKind, Read};

// /!\ .PAK from Quake2 is stocked on 32bit var !

pub struct PakEntry {
    pub filename:   String,
    pub offset:     u32,
    pub size:       u32
}

struct PakHeader {
    pub ident:      i32,
    pub dir_offset: u32,
    pub dir_size:   u32
}

pub struct PAK {
    header :        PakHeader,
    files :         Vec<PakEntry>,
    file :          File
}

impl PAK {
    pub fn open(filepath: String) -> Result<PAK, Error> {
        const magic_header :  u32 = (75 << 24)+(67 << 16)+(65 <<8)+80;

        let mut file : File = File::open(filepath).unwrap();
        let file_len : u64 = file.metadata().unwrap().len();

        let mut data = vec![0; file_len as usize];
        file.read(&mut data).expect("");

        let mut header : PakHeader = PakHeader {
            ident: 0,
            dir_offset: 0,
            dir_size: 0
        };

        let ident = data[0..4].try_into().unwrap();
        let dir_offset = data[4..8].try_into().unwrap();
        let dir_size = data[8..12].try_into().unwrap();

        header.ident = i32::from_le_bytes(ident);
        header.dir_offset = u32::from_le_bytes(dir_offset);
        header.dir_size = u32::from_le_bytes(dir_size) >> 6;


        if header.ident as u32 != magic_header {
            return Err(Error::new(ErrorKind::Other, "PAK is busy !"))
        }

        file.seek(SeekFrom::Start(header.dir_offset as u64)).unwrap();
        file.read(&mut data).expect("");

        let files : Vec<PakEntry> = PAK::read_pak_entries(&data[0..(header.dir_size as usize * std::mem::size_of::<PakEntry>()) as usize]);

        Ok(PAK {
            header: header,
            files: files,
            file : file
        })

    }

    fn read_pak_entries(_data: &[u8]) -> Vec<PakEntry>{
        let mut data = _data;
        let mut pak_files : Vec<PakEntry> = Vec::new();

        loop {
            //Check if data array is empty
            if data.len() == 0 {
                break;
            }

            let filename_bytes = data[0..56].to_vec();
            let size_bytes = data[56..60].try_into().unwrap();
            let offset_bytes = data[60..64].try_into().unwrap();

            let filename = String::from_utf8(filename_bytes).unwrap();
            let size = u32::from_le_bytes(size_bytes);
            let offset = u32::from_le_bytes(offset_bytes);

            println!("{}", filename);

            pak_files.push(PakEntry{
                filename: filename,
                size: size,
                offset: offset
            });

            data = &data[64..data.len()];
        }

        pak_files

    }

    pub fn get_files(&self, filepath: String) -> Option<&PakEntry> {
        for i in 0..self.files.len() as usize {
            if self.files[i].filename == filepath {
                return Some(&self.files[i]);
            }
        }
        None
    }
}
