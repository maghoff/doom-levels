use std::io::Write;

use wad_map::{BoundingBox, Linedef, Vertex};

pub fn generate_svg(
    mut out: impl Write,
    vertexes: &[Vertex],
    linedefs: &[Linedef],
) -> std::io::Result<()> {
    let mut bbox = BoundingBox::from(vertexes);
    bbox.grow(20);

    writeln!(
        out,
        r#"<svg viewBox="{} {} {} {}" xmlns="http://www.w3.org/2000/svg">"#,
        bbox.left(),
        -bbox.bottom(),
        bbox.width(),
        bbox.height()
    )?;
    writeln!(out, "<style>{}</style>", include_str!("svg.css"))?;
    for linedef in linedefs {
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
