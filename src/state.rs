use enigmap::renderers::colors::ColorMap;
use enigmap::generators::*;
use enigmap::{HexMap, HexType};

#[derive(Debug, Clone)]
pub struct State {
    pub size_x: u32,
    pub size_y: u32,
    pub random_seed: bool,
    pub seed: u32,
    pub gen: Generator,
    pub ren: Renderer,
    pub map: HexMap,
    pub color_map: ColorMap,
    pub zoom_level: i32
}

impl State {
    pub fn set_size_x(&mut self, new_val: u32) {
        if new_val != self.size_x {
            self.size_x = new_val;
            self.map.remap(self.size_x, self.size_y, HexType::Water);
            // this lags the UI for some reason
            //gl_map_resized(self.map.absolute_size_x, self.map.absolute_size_y);
        }
    }

    pub fn set_size_y(&mut self, new_val: u32) {
        if new_val != self.size_y {
            self.size_y = new_val;
            self.map.remap(self.size_x, self.size_y, HexType::Water);
            //gl_map_resized(self.map.absolute_size_x, self.map.absolute_size_y);
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            size_x: 100,
            size_y: 75,
            gen: Generator::Circle(Circle::default()),
            ren: Renderer::Ogl,
            random_seed: true,
            seed: 0,
            map: HexMap::new(100, 75),
            color_map: ColorMap::default(),
            zoom_level: 0
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Generator {
    Circle(Circle),
    Island(Islands),
    Inland(Inland)
}

#[derive(Debug, Copy, Clone)]
pub enum Renderer {
    Ogl,
    OglTextured
}
