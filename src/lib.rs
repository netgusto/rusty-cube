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

    fn zozo(v: f32);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let f = 5.25;
    log("Hello, cube!");
    zozo(f);
}

#[wasm_bindgen]
pub struct Universe {
    vertices: Vec<Vertex2>
}

#[wasm_bindgen]
pub struct Vertex2 {
    x: f32,
    y: f32
}

#[wasm_bindgen]
impl Universe {

    pub fn new() -> Universe {

        let mut vertices = Vec::new();

        vertices.push(Vertex2{ x: 0.0, y: 0.0 });
        vertices.push(Vertex2{ x: 0.0, y: 100.0 });
        vertices.push(Vertex2{ x: 100.0, y: 200.0 });
        vertices.push(Vertex2{ x: 10.0/3.0, y: 200.0 });

        Universe {
            vertices: vertices
        }
    }

    pub fn tick(&mut self) {
        log("ticked!");
    }

    pub fn vertices(&self) -> *const Vertex2 {
        self.vertices.as_ptr()
    }

    pub fn nbVertices(&self) -> usize {
        self.vertices.len()
    }
}