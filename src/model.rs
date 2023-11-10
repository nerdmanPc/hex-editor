#[allow(unused)]
pub mod hexagon;
#[allow(unused)]
pub mod point;
#[allow(unused)]
pub mod tools;
#[allow(unused)]
pub mod layout;

#[cfg(test)]
mod tests;

pub use layout::{
    Layout,
    LayoutTool,
    LAYOUT_ORIENTATION_POINTY,
};
pub use hexagon::{
    Hex,
    HexMath,
    HexRound,
};
pub use point::Point;

use std::collections::{HashMap, hash_map};

//use self::hexagon::HexMath;

pub struct Grid {
    layout: Layout,
    data: HashMap<Hex, egui::Color32>
}

impl Grid {
    pub fn make_rhombus(min: Hex, max: Hex) -> Self {
        let mut instance = Self::default();
        for q in min.q() ..= max.q() {
            for r in min.r() ..= max.r() {
                let key = Hex::new(q, r);
                instance.data.insert(key, egui::Color32::default());
            }
        }
        instance
    }

    pub fn make_triangle(min: Hex, size: i32) -> Self {
        let mut instance = Self::default();
        for q in min.q() ..=  min.q() + size {
            for r in min.r() ..= min.r() + size - q {
                let key = Hex::new(q, r);
                instance.data.insert(key, egui::Color32::default());
            }
        }
        instance
    }

    pub fn make_hex(center: Hex, size: i32) -> Self {
        let mut instance = Self::default();
        for q in -size ..= size {
            for r in -size ..= size {
                let s = -q-r;
                if (-size <= s) && (s <= size) {
                    let key = center.add(Hex::new(q, r));
                    instance.data.insert(key, egui::Color32::default());
                }
            }
        }
        instance
    }

    pub fn paint_cell(&mut self, cell: Hex, color: egui::Color32) {
        self.data.insert(cell, color);
    }

    pub fn sample_cell(&self, x: f64, y: f64) -> Hex {
        let fractional_coord = LayoutTool::pixel_to_hex(self.layout, Point { x, y });
        fractional_coord.round()
    }

    pub fn cells(&self) -> hash_map::Iter<Hex, egui::Color32> {
        self.data.iter()
    }

    pub fn layout(&self) -> Layout {
        self.layout
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
