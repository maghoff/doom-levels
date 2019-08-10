use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Read, Result};

pub struct Thing {
    pub x: i16,
    pub y: i16,
    pub ang: i16,
    pub thing_type: u16,
    pub flags: u16,
}

pub fn read_thing<R: Read>(r: &mut R) -> Result<Thing> {
    Ok(Thing {
        x: r.read_i16::<LittleEndian>()?,
        y: r.read_i16::<LittleEndian>()?,
        ang: r.read_i16::<LittleEndian>()?,
        thing_type: r.read_u16::<LittleEndian>()?,
        flags: r.read_u16::<LittleEndian>()?,
    })
}

pub fn parse_things<R: Read>(r: &mut R) -> Result<Vec<Thing>> {
    let mut res = vec![];

    loop {
        match read_thing(r) {
            Ok(v) => res.push(v),
            Err(ref err) if err.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(res),
            Err(err) => return Err(err),
        }
    }
}
