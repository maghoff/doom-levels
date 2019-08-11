use crate::*;

pub struct Map {
    pub things: Vec<Thing>,
    pub linedefs: Vec<Linedef>,
    pub sidedefs: Vec<Sidedef>,
    pub vertexes: Vec<Vertex>,
    pub line_segments: Vec<LineSegment>,
    pub subsectors: Vec<Subsector>,
    pub nodes: Vec<Node>,
    pub sectors: Vec<Sector>,
    // pub reject: ,
    // pub blockmap: ,
}

pub fn map_from_slice(map: &wad::WadSlice) -> Result<Map, Box<dyn std::error::Error>> {
    let mut things = map.by_id(b"THINGS").ok_or("Cannot find THINGS")?;
    let things = parse_things(&mut things)?;

    let mut linedefs = map.by_id(b"LINEDEFS").ok_or("Cannot find LINEDEFS")?;
    let linedefs = parse_linedefs(&mut linedefs)?;

    let mut sidedefs = map.by_id(b"SIDEDEFS").ok_or("Cannot find SIDEDEFS")?;
    let sidedefs = parse_sidedefs(&mut sidedefs)?;

    let mut vertexes = map.by_id(b"VERTEXES").ok_or("Cannot find VERTEXES")?;
    let vertexes = parse_vertexes(&mut vertexes)?;

    let mut line_segments = map.by_id(b"SEGS").ok_or("Cannot find SEGS")?;
    let line_segments = parse_segs(&mut line_segments)?;

    let mut subsectors = map.by_id(b"SSECTORS").ok_or("Cannot find SSECTORS")?;
    let subsectors = parse_ssectors(&mut subsectors)?;

    let mut nodes = map.by_id(b"NODES").ok_or("Cannot find NODES")?;
    let nodes = parse_nodes(&mut nodes)?;

    let mut sectors = map.by_id(b"SECTORS").ok_or("Cannot find SECTORS")?;
    let sectors = parse_sectors(&mut sectors)?;

    Ok(Map {
        things,
        linedefs,
        sidedefs,
        vertexes,
        line_segments,
        subsectors,
        nodes,
        sectors,
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
