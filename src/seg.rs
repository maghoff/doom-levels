use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

#[derive(Debug)]
pub struct LineSegment {
    pub start_vertex: u16,
    pub end_vertex: u16,
    pub angle: u16,
    pub linedef: u16,
    pub direction: u16,
    pub offset: u16,
}

pub fn read_seg<R: Read>(r: &mut R) -> Result<LineSegment, std::io::Error> {
    Ok(LineSegment {
        start_vertex: r.read_u16::<LittleEndian>()?,
        end_vertex: r.read_u16::<LittleEndian>()?,
        angle: r.read_u16::<LittleEndian>()?,
        linedef: r.read_u16::<LittleEndian>()?,
        direction: r.read_u16::<LittleEndian>()?,
        offset: r.read_u16::<LittleEndian>()?,
    })
}

pub fn parse_segs<R: Read>(r: &mut R) -> Result<Vec<LineSegment>, std::io::Error> {
    let mut res = vec![];

    loop {
        match read_seg(r) {
            Ok(v) => res.push(v),
            Err(ref err) if err.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(res),
            Err(err) => return Err(err),
        }
    }
}
