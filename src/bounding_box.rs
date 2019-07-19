use std::cmp::{max, min};

use crate::Vertex;

pub struct BoundingBox {
    pub top: i16,
    pub left: i16,
    pub bottom: i16,
    pub right: i16,
}

impl BoundingBox {
    pub fn grow(&mut self, border: i16) {
        self.top -= border;
        self.left -= border;
        self.bottom += border;
        self.right += border;
    }

    pub fn width(&self) -> i16 {
        self.right - self.left
    }

    pub fn height(&self) -> i16 {
        self.bottom - self.top
    }

    pub fn join(a: &BoundingBox, b: &BoundingBox) -> BoundingBox {
        BoundingBox {
            top: min(a.top, b.top),
            left: min(a.left, b.left),
            bottom: max(a.bottom, b.bottom),
            right: max(a.right, b.right),
        }
    }
}

impl From<&Vertex> for BoundingBox {
    fn from(src: &Vertex) -> Self {
        BoundingBox {
            top: src.y,
            left: src.x,
            bottom: src.y,
            right: src.x,
        }
    }
}

impl<'a, I> From<I> for BoundingBox
where
    I: IntoIterator<Item = &'a Vertex>
{
    fn from(src: I) -> Self {
        let mut vertexes = src.into_iter();

        let first = vertexes.next().expect("src cannot be empty");
        let mut bb = BoundingBox::from(first);

        for vertex in vertexes {
            bb = BoundingBox::join(&bb, &vertex.into());
        }

        bb
    }
}
