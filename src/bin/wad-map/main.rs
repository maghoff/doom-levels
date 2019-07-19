extern crate wad_map;

use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

use wad_map::*;

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

    let mut bbox = BoundingBox::from(&vertexes);
    bbox.grow(20);

    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    writeln!(
        out,
        r#"<svg viewBox="{} {} {} {}" xmlns="http://www.w3.org/2000/svg">"#,
        bbox.left(),
        -bbox.bottom(),
        bbox.width(),
        bbox.height()
    )?;
    writeln!(out, "<style>{}</style>", include_str!("svg.css"))?;
    for linedef in &linedefs {
        let a = &vertexes[linedef.a as usize];
        let b = &vertexes[linedef.b as usize];

        let portal = linedef.left_sidedef.is_some() && linedef.right_sidedef.is_some();

        let class = if portal { r#" class="portal""# } else { "" };

        writeln!(
            out,
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}"{} />"#,
            a.x, -a.y, b.x, -b.y, class
        )?;
    }
    writeln!(out, r#"</svg>"#)?;

    Ok(())
}
