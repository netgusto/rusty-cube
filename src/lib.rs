extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator. 
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// #[wasm_bindgen]
// pub fn greet() {
//     let f = 5.25;
//     log("Hello, cube!");
//     zozo(f);
// }

#[wasm_bindgen]
pub struct Universe {
    ticknum: i32,
    geometry: [Vertex2; 4],
    vertices: [Vertex2; 4]
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Vertex2 {
    x: f32,
    y: f32
}

#[wasm_bindgen]
impl Universe {

    pub fn new() -> Universe {

        // let mut vertices = Vec::new();
        let geometry: [Vertex2; 4] = [
            Vertex2{ x: 0.0, y: 0.0 },
            Vertex2{ x: 0.0, y: 100.0 },
            Vertex2{ x: 100.0, y: 0.0 },
            Vertex2{ x: 100.0, y: 100.0 },
        ];

        let mut vertices: [Vertex2; 4] = [
            Vertex2{ x: 0.0, y: 0.0 },
            Vertex2{ x: 0.0, y: 0.0 },
            Vertex2{ x: 0.0, y: 0.0 },
            Vertex2{ x: 0.0, y: 0.0 },
        ];

        vertices.clone_from_slice(&geometry);

        Universe {
            ticknum: 0,
            geometry: geometry,
            vertices: vertices
        }
    }

    pub fn tick(&mut self) {

        let rotation_center = Vertex2{ x: 300.0, y: 300.0 };
        let theta = self.ticknum as f32 / 30.0;

        self.rotate_geometry(&rotation_center, theta);

        self.ticknum += 1;
    }

    pub fn vertices(&self) -> *const Vertex2 {
        self.vertices.as_ptr()
    }

    pub fn nb_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn size_vertex(&self) -> usize {
        std::mem::size_of::<Vertex2>()
    }

    pub fn ticknum(&self) -> i32 {
        self.ticknum
    }

    fn rotate_geometry(&mut self, center: &Vertex2, theta: f32) {
        // self.vertices[0].x += 10.0/16.0;
        for (i, p) in self.vertices.iter_mut().enumerate() {
            let geo = self.geometry[i];
            p.x = center.x + (geo.x * theta.cos() - geo.y * theta.sin());
            p.y = center.y + (geo.y * theta.cos() + geo.x * theta.sin());
        }
    }
}