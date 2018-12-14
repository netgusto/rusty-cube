import * as wasm from "cube";

// Import the WebAssembly memory at the top of the file.
import { memory } from "cube/cube_bg";

window.zozo = (f64) => {
    console.log("ZOZO!", Math.PI, f64);
}

wasm.greet();

const canvas = document.getElementById('canvas');
canvas.height = 1000;
canvas.width = 1000;

const ctx = canvas.getContext("2d");
const raf = window.requestAnimationFrame;

const geometry = [
    [0, 0],
    [0, 100],
    [100, 200],
];

const rotationCenter = [300, 300];

let i = 0;

const tick = (geometry) => {
    const theta = i / 30;  // radians
    i++;
    return rotateGeometry(geometry, rotationCenter, theta);
};

const render = (vertices) => {

    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.strokeStyle = '#000000';
    ctx.fillStyle = '#000000';

    vertices.map(p => ctx.fillRect(p[0], p[1], 3, 3));
};

const universe = wasm.Universe.new();
const verticesPtr = universe.vertices();
const vertices = new Float32Array(memory.buffer, verticesPtr, universe.nbVertices() * 2);
console.log(vertices);

const animate = () => {
    universe.tick();
    // render(vertices);
    raf(animate);
}

animate();

function rotate2D(center, point, theta) {
    return [
        center[0] + (point[0] * Math.cos(theta) - point[1] * Math.sin(theta)),
        center[1] + (point[1] * Math.cos(theta) + point[0] * Math.sin(theta)),
    ];
}

function rotateGeometry(geometry, center, theta) {
    return geometry.map(p => rotate2D(center, p, theta));
}