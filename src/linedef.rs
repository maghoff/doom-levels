use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

#[derive(Debug)]
pub struct Linedef {
    pub a: u16,
    pub b: u16,
    pub flags: u16,
    pub special_type: u16,
    pub sector_tag: u16,
    pub right_sidedef: Option<u16>,
    pub left_sidedef: Option<u16>,
}

pub fn read_linedef<R: Read>(r: &mut R) -> Result<Linedef, std::io::Error> {
    Ok(Linedef {
        a: r.read_u16::<LittleEndian>()?,
        b: r.read_u16::<LittleEndian>()?,
        flags: r.read_u16::<LittleEndian>()?,
        special_type: r.read_u16::<LittleEndian>()?,
        sector_tag: r.read_u16::<LittleEndian>()?,
        right_sidedef: match r.read_i16::<LittleEndian>()? {
            -1 => None,
            x => Some(x as u16),
        },
        left_sidedef: match r.read_i16::<LittleEndian>()? {
            -1 => None,
            x => Some(x as u16),
        },
    })
}

pub fn parse_linedefs<R: Read>(r: &mut R) -> Result<Vec<Linedef>, std::io::Error> {
    let mut res = vec![];

    loop {
        match read_linedef(r) {
            Ok(v) => res.push(v),
            Err(ref err) if err.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(res),
            Err(err) => return Err(err),
        }
    }
}
