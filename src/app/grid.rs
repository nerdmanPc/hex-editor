mod hex_utils; pub use hex_utils::*;

use {
    std::{
        collections::{HashMap, hash_map}, 
        vec::IntoIter, 
        iter::Map
    }, 
    egui::Color32
};

pub struct Grid {
    layout: Layout,
    data: HashMap<Hex, Color32>
}

impl Grid {

    pub fn _make_rhombus(min: impl Into<Hex>, max: impl Into<Hex>) -> Self {
        let (min, max): (Hex, Hex) = (min.into(), max.into());
        let mut instance = Self::default();
        for q in min.q() ..= max.q() {
            for r in min.r() ..= max.r() {
                let key = Hex::new(q, r);
                instance.data.insert(key, Color32::default());
            }
        }
        instance
    }

    pub fn _make_triangle(min: impl Into<Hex>, size: i32) -> Self {
        let min: Hex = min.into();
        let mut instance = Self::default();
        for q in min.q() ..=  min.q() + size {
            for r in min.r() ..= min.r() + size - q {
                let key = Hex::new(q, r);
                instance.data.insert(key, Color32::default());
            }
        }
        instance
    }

    pub fn make_hex(center: impl Into<Hex>, size: i32) -> Self {
        let center: Hex = center.into();
        let mut instance = Self::default();
        for q in -size ..= size {
            for r in -size ..= size {
                let s = -q-r;
                if (-size <= s) && (s <= size) {
                    let key = center.add(Hex::new(q, r));
                    instance.data.insert(key, Color32::default());
                }
            }
        }
        instance
    }

    pub fn paint_cell(&mut self, cell: impl Into<Hex>, color: impl Into<Color32>) {
        self.data.insert(cell.into(), color.into());
    }

    pub fn sample_cell(&self, pos: impl Into<Point>) -> Hex {

        let fractional_coord = LayoutTool::pixel_to_hex(self.layout, pos.into());
        fractional_coord.round()
    }

    pub fn polygon_corners(&self, key: Hex) -> Map<IntoIter<Point>, fn(Point)->[f32; 2]>//Box<dyn Iterator<Item = [f32; 2]>>
    {
        let convert_point: fn(Point) -> [f32; 2] = |point: Point| {
            [point.x as f32, point.y as f32]
        };
        LayoutTool::polygon_corners(self.layout, key).into_iter().map(convert_point)
    }

    pub fn cells(&self) -> hash_map::Iter<Hex, Color32> {
        self.data.iter()
    }
}


impl Default for Grid {
    fn default() -> Self {
        let layout = Layout {
            orientation: LAYOUT_ORIENTATION_POINTY,
            size: Point { x:0.1, y:0.1 }, 
            origin: Point { x: 0.0, y: 0.0 },
        };
        let data = HashMap::new();
        Self {
            layout,
            data,
        }
    }
}
