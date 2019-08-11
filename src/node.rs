use crate::BoundingBox;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

#[derive(Debug)]
pub struct Node {
    pub x: i16,
    pub y: i16,
    pub dx: i16,
    pub dy: i16,
    pub right_bb: BoundingBox,
    pub left_bb: BoundingBox,
    pub right_child: u16,
    pub left_child: u16,
}

pub fn read_node<R: Read>(r: &mut R) -> Result<Node, std::io::Error> {
    Ok(Node {
        x: r.read_i16::<LittleEndian>()?,
        y: r.read_i16::<LittleEndian>()?,
        dx: r.read_i16::<LittleEndian>()?,
        dy: r.read_i16::<LittleEndian>()?,
        right_bb: BoundingBox::new(
            r.read_i16::<LittleEndian>()?,
            r.read_i16::<LittleEndian>()?,
            r.read_i16::<LittleEndian>()?,
            r.read_i16::<LittleEndian>()?,
        ),
        left_bb: BoundingBox::new(
            r.read_i16::<LittleEndian>()?,
            r.read_i16::<LittleEndian>()?,
            r.read_i16::<LittleEndian>()?,
            r.read_i16::<LittleEndian>()?,
        ),
        right_child: r.read_u16::<LittleEndian>()?,
        left_child: r.read_u16::<LittleEndian>()?,
    })
}

pub fn parse_nodes<R: Read>(r: &mut R) -> Result<Vec<Node>, std::io::Error> {
    let mut res = vec![];

    loop {
        match read_node(r) {
            Ok(v) => res.push(v),
            Err(ref err) if err.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(res),
            Err(err) => return Err(err),
        }
    }
}
