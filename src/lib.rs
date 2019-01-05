#![allow(dead_code)]

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
pub struct Universe {
    ticknum: i32,
    vertices: Vec<Vertex3>
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Vertex3 {
    x: f32,
    y: f32,
    z: f32
}

#[wasm_bindgen]
impl Universe {
    
    pub fn new() -> Universe {

        let geometry: Vec<Vertex3> = vec!(
            Vertex3{ x: -100.0, y: -100.0, z: -100.0 },
            Vertex3{ x: -100.0, y: -100.0, z: 100.0 },
            Vertex3{ x: -100.0, y: 100.0, z: -100.0 },
            Vertex3{ x: -100.0, y: 100.0, z: 100.0 },
            Vertex3{ x: 100.0, y: -100.0, z: -100.0 },
            Vertex3{ x: 100.0, y: -100.0, z: 100.0 },
            Vertex3{ x: 100.0, y: 100.0, z: -100.0 },
            Vertex3{ x: 100.0, y: 100.0, z: 100.0 },
        );

        let mut vertices: Vec<Vertex3> = Vec::new();
        for i in 0..geometry.len() {
            let v = geometry[i];
            vertices.push(Vertex3{ x: v.x, y: v.y, z: v.z });
        }

        Universe {
            ticknum: 0,
            vertices: vertices
        }
    }

    pub fn tick(&mut self) {

        rotate_z3d(&mut self.vertices, 1.0/90.0);
        rotate_y3d(&mut self.vertices, 1.0/60.0);
        rotate_x3d(&mut self.vertices, -1.0/30.0);

        self.ticknum += 1;
    }

    pub fn vertices(&self) -> *const Vertex3 {
        self.vertices.as_ptr()
    }

    pub fn nb_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn size_vertex(&self) -> usize {
        std::mem::size_of::<Vertex3>()
    }

    pub fn ticknum(&self) -> i32 {
        self.ticknum
    }
}

fn rotate_z3d(vertices: &mut Vec<Vertex3>, theta: f32) {
    let sin_t = theta.sin();
    let cos_t = theta.cos();

    for v in vertices.iter_mut() {
        let x = v.x;
        let y = v.y;
        v.x = x * cos_t - y * sin_t;
        v.y = y * cos_t + x * sin_t;
    }
}

fn rotate_y3d(vertices: &mut Vec<Vertex3>, theta: f32) {
    let sin_t = theta.sin();
    let cos_t = theta.cos();

    for v in vertices.iter_mut() {
        let x = v.x;
        let z = v.z;
        v.x = x * cos_t - z * sin_t;
        v.z = z * cos_t + x * sin_t;
    }
}

fn rotate_x3d(vertices: &mut Vec<Vertex3>, theta: f32) {
    let sin_t = theta.sin();
    let cos_t = theta.cos();

    for v in vertices.iter_mut() {
        let y = v.y;
        let z = v.z;
        v.y = y * cos_t - z * sin_t;
        v.z = z * cos_t + y * sin_t;
    }
}