use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

#[derive(Debug)]
pub struct Subsector {
    pub seg_count: u16,
    pub first_seg: u16,
}

pub fn read_subsector<R: Read>(r: &mut R) -> Result<Subsector, std::io::Error> {
    Ok(Subsector {
        seg_count: r.read_u16::<LittleEndian>()?,
        first_seg: r.read_u16::<LittleEndian>()?,
    })
}

pub fn parse_ssectors<R: Read>(r: &mut R) -> Result<Vec<Subsector>, std::io::Error> {
    let mut res = vec![];

    loop {
        match read_subsector(r) {
            Ok(v) => res.push(v),
            Err(ref err) if err.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(res),
            Err(err) => return Err(err),
        }
    }
}
