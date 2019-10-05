use enigmap::renderers::colors::ColorMap;
use enigmap::generators::*;
use enigmap::HexMap;

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
