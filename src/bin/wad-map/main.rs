extern crate wad_map;

use std::path::PathBuf;
use structopt::StructOpt;

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

    let map = wad_map::read_map(&wad.as_slice(), &opt.map)?;

    let stdout = std::io::stdout();
    let out = stdout.lock();

    svg::generate_svg(out, &map.vertexes, &map.linedefs)?;

    Ok(())
}
