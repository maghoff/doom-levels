use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

#[derive(Debug)]
pub struct Sector {
    pub floor_height: i16,
    pub ceil_height: i16,
    pub floor_texture: [u8; 8],
    pub ceil_texture: [u8; 8],
    pub light_level: i16,
    pub sector_type: i16,
    pub tag: u16,
}

pub fn read_sector<R: Read>(r: &mut R) -> Result<Sector, std::io::Error> {
    let mut s = Sector {
        floor_height: r.read_i16::<LittleEndian>()?,
        ceil_height: r.read_i16::<LittleEndian>()?,
        floor_texture: [0; 8],
        ceil_texture: [0; 8],
        light_level: 0,
        sector_type: 0,
        tag: 0,
    };

    r.read_exact(&mut s.floor_texture)?;
    r.read_exact(&mut s.ceil_texture)?;

    s.light_level = r.read_i16::<LittleEndian>()?;
    s.sector_type = r.read_i16::<LittleEndian>()?;
    s.tag = r.read_u16::<LittleEndian>()?;

    Ok(s)
}

pub fn parse_sectors<R: Read>(r: &mut R) -> Result<Vec<Sector>, std::io::Error> {
    let mut res = vec![];

    loop {
        match read_sector(r) {
            Ok(v) => res.push(v),
            Err(ref err) if err.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(res),
            Err(err) => return Err(err),
        }
    }
}
