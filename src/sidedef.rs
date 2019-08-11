use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

#[derive(Debug)]
pub struct Sidedef {
    pub x_offset: i16,
    pub y_offset: i16,
    pub upper_texture: [u8; 8],
    pub lower_texture: [u8; 8],
    pub middle_texture: [u8; 8],
    pub sector_id: u16,
}

pub fn read_sidedef<R: Read>(r: &mut R) -> Result<Sidedef, std::io::Error> {
    let mut s = Sidedef {
        x_offset: r.read_i16::<LittleEndian>()?,
        y_offset: r.read_i16::<LittleEndian>()?,
        upper_texture: [0; 8],
        lower_texture: [0; 8],
        middle_texture: [0; 8],
        sector_id: 0,
    };

    r.read_exact(&mut s.upper_texture)?;
    r.read_exact(&mut s.lower_texture)?;
    r.read_exact(&mut s.middle_texture)?;

    s.sector_id = r.read_u16::<LittleEndian>()?;

    Ok(s)
}

pub fn parse_sidedefs<R: Read>(r: &mut R) -> Result<Vec<Sidedef>, std::io::Error> {
    let mut res = vec![];

    loop {
        match read_sidedef(r) {
            Ok(v) => res.push(v),
            Err(ref err) if err.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(res),
            Err(err) => return Err(err),
        }
    }
}
