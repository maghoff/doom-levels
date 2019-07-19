extern crate wad_map;

use std::path::PathBuf;
use structopt::StructOpt;

use wad_map::{parse_linedefs, parse_vertexes};

mod svg;

#[derive(Debug, StructOpt)]
#[structopt(name = "wad-map", about = "Do stuff with map data in WAD files")]
struct Opt {
    /// Input WAD file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Which map to read from the WAD file, eg E1M1 for DOOM 1 or MAP01 for
    /// DOOM 2
    map: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let wad = wad::load_wad_file(&opt.input)?;

    let map =
        wad::EntryId::from_str(&opt.map).ok_or_else(|| format!("Invalid lump ID {:?}", opt.map))?;
    let map = wad.index_of(map).ok_or("Cannot find E1M1")?;
    let map = wad.slice(map + 1..);

    let vertexes = map.by_id(b"VERTEXES").ok_or("Cannot find VERTEXES")?;
    let vertexes = parse_vertexes(&mut vertexes.clone())?;

    let linedefs = map.by_id(b"LINEDEFS").ok_or("Cannot find LINEDEFS")?;
    let linedefs = parse_linedefs(&mut linedefs.clone())?;

    let stdout = std::io::stdout();
    let out = stdout.lock();

    svg::generate_svg(out, &vertexes, &linedefs)?;

    Ok(())
}
