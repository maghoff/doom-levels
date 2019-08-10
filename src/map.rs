use crate::*;

pub struct Map {
    pub things: Vec<Thing>,
    pub linedefs: Vec<Linedef>,
    // pub sidedefs: ,
    pub vertexes: Vec<Vertex>,
    // pub segs: ,
    // pub ssectors: ,
    // pub nodes: ,
    // pub sectors: ,
    // pub reject: ,
    // pub blockmap: ,
}

pub fn map_from_slice(map: &wad::WadSlice) -> Result<Map, Box<dyn std::error::Error>> {
    let mut things = map.by_id(b"THINGS").ok_or("Cannot find THINGS")?;
    let things = parse_things(&mut things)?;

    let mut vertexes = map.by_id(b"VERTEXES").ok_or("Cannot find VERTEXES")?;
    let vertexes = parse_vertexes(&mut vertexes)?;

    let mut linedefs = map.by_id(b"LINEDEFS").ok_or("Cannot find LINEDEFS")?;
    let linedefs = parse_linedefs(&mut linedefs)?;

    Ok(Map {
        things,
        linedefs,
        vertexes,
    })
}

pub fn read_map(wad: &wad::WadSlice, map_name: &str) -> Result<Map, Box<dyn std::error::Error>> {
    let map = wad::EntryId::from_str(&map_name)
        .ok_or_else(|| format!("Invalid lump ID {:?}", map_name))?;
    let map = wad
        .index_of(map)
        .ok_or_else(|| format!("Cannot find {}", map_name))?;
    let map = wad.slice(map + 1..map + 11);

    map_from_slice(&map)
}
