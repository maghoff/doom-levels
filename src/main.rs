use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;

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

#[derive(Debug)]
struct Vertex {
    x: i16,
    y: i16,
}

fn read_vertex<R: Read>(r: &mut R) -> Result<Vertex, std::io::Error> {
    Ok(Vertex {
        x: r.read_i16::<LittleEndian>()?,
        y: r.read_i16::<LittleEndian>()?,
    })
}

fn parse_vertexes<R: Read>(r: &mut R) -> Result<Vec<Vertex>, std::io::Error> {
    let mut res = vec![];

    loop {
        match read_vertex(r) {
            Ok(v) => res.push(v),
            Err(ref err) if err.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(res),
            Err(err) => return Err(err),
        }
    }
}

#[derive(Debug)]
struct Linedef {
    a: u16,
    b: u16,
    flags: u16,
    special_type: u16,
    sector_tag: u16,
    right_sidedef: Option<u16>,
    left_sidedef: Option<u16>,
}

fn read_linedef<R: Read>(r: &mut R) -> Result<Linedef, std::io::Error> {
    Ok(Linedef {
        a: r.read_u16::<LittleEndian>()?,
        b: r.read_u16::<LittleEndian>()?,
        flags: r.read_u16::<LittleEndian>()?,
        special_type: r.read_u16::<LittleEndian>()?,
        sector_tag: r.read_u16::<LittleEndian>()?,
        right_sidedef: match r.read_i16::<LittleEndian>()? {
            -1 => None,
            x => Some(x as u16),
        },
        left_sidedef: match r.read_i16::<LittleEndian>()? {
            -1 => None,
            x => Some(x as u16),
        },
    })
}

fn parse_linedefs<R: Read>(r: &mut R) -> Result<Vec<Linedef>, std::io::Error> {
    let mut res = vec![];

    loop {
        match read_linedef(r) {
            Ok(v) => res.push(v),
            Err(ref err) if err.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(res),
            Err(err) => return Err(err),
        }
    }
}

struct BoundingBox {
    top: i16,
    left: i16,
    bottom: i16,
    right: i16,
}

impl BoundingBox {
    fn grow(&mut self, border: i16) {
        self.top -= border;
        self.left -= border;
        self.bottom += border;
        self.right += border;
    }

    fn width(&self) -> i16 {
        self.right - self.left
    }

    fn height(&self) -> i16 {
        self.bottom - self.top
    }
}

fn calculate_bounding_box<'a>(vertexes: impl IntoIterator<Item = &'a Vertex>) -> BoundingBox {
    use std::cmp::{max, min};

    let mut vertexes = vertexes.into_iter();

    let first = vertexes.next().expect("vertexes cannot be empty");
    let mut bb = BoundingBox {
        top: first.y,
        left: first.x,
        bottom: first.y,
        right: first.x,
    };

    for vertex in vertexes {
        bb.top = min(bb.top, vertex.y);
        bb.left = min(bb.left, vertex.x);
        bb.bottom = max(bb.bottom, vertex.y);
        bb.right = max(bb.right, vertex.x);
    }

    bb
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

    let mut bbox = calculate_bounding_box(&vertexes);
    bbox.grow(20);

    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    writeln!(
        out,
        r#"<svg viewBox="{} {} {} {}" xmlns="http://www.w3.org/2000/svg">"#,
        bbox.left,
        -bbox.bottom,
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
