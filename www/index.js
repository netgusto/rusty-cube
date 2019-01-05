import * as wasm from "cube";
import * as jBinary from 'jbinary';

// Import the WebAssembly memory at the top of the file.
import { memory } from "cube/cube_bg";

const universe = wasm.Universe.new();
const verticesPtr = universe.vertices();
// const vertices = new Float32Array(memory.buffer, verticesPtr, universe.nb_vertices() * 2);
// console.log(vertices);

const myTypeset = {
    'jBinary.all': 'Vertices',

    // wasm is Little-Endian; https://github.com/WebAssembly/design/blob/master/Portability.md
    'jBinary.littleEndian': true,
    Vertex3: {
        x: 'float',
        y: 'float',
        z: 'float',
    },
    Vertices: ['array', 'Vertex3'],
};

const binary = new jBinary(memory.buffer, myTypeset);
const vertices = binary.slice(verticesPtr, verticesPtr + universe.nb_vertices() * universe.size_vertex());
console.log(vertices.readAll(), universe.nb_vertices());

const canvas = document.getElementById('canvas');
canvas.height = 1000;
canvas.width = 1000;

const ctx = canvas.getContext("2d");
const raf = window.requestAnimationFrame;

const render = () => {

    universe.tick();

    ctx.strokeStyle = '#000000';
    ctx.fillStyle = '#FF0000';
    ctx.lineWidth = 2;
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    const nbVertices = universe.nb_vertices();
    vertices.seek(0);

    for (let i = 0; i < nbVertices; i++) {
        const p = vertices.read('Vertex3');
        // console.log(p);
        const proximity = (p.z + 200) / 200;
        ctx.fillStyle = 'rgba(255, 0, 0, ' + proximity + ')'
        const diameter = 5 + proximity * 2;
        // const diameter = 4;
        // ctx.fillRect(p.x + 300, p.y + 300, diameter, diameter);
        ctx.beginPath();
        ctx.arc(p.x + 300, p.y + 300, diameter, 0, 2*Math.PI, false);
        ctx.fill();
    }
};

(function() {
    const gameloop = () => {
        render();
        raf(gameloop);
    };
    gameloop();
})();
