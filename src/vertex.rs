use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

#[derive(Debug)]
pub struct Vertex {
    pub x: i16,
    pub y: i16,
}

pub fn read_vertex<R: Read>(r: &mut R) -> Result<Vertex, std::io::Error> {
    Ok(Vertex {
        x: r.read_i16::<LittleEndian>()?,
        y: r.read_i16::<LittleEndian>()?,
    })
}

pub fn parse_vertexes<R: Read>(r: &mut R) -> Result<Vec<Vertex>, std::io::Error> {
    let mut res = vec![];

    loop {
        match read_vertex(r) {
            Ok(v) => res.push(v),
            Err(ref err) if err.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(res),
            Err(err) => return Err(err),
        }
    }
}
