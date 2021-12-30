import init, { decode, encode } from "../pkg/wasm_gif.js";
// Fetch a gif online first
const gif = await init();
let req = await fetch("https://media.giphy.com/media/wJrZCVwiPUwE0mPIjw/giphy-downsized-large.gif");
// get the `uint8 array` from the request
let ab = await req.arrayBuffer();
let u8 = new Uint8Array(ab);
console.time('wasm decode');
console.log(decode(u8));
console.timeEnd('wasm decode');
const canvas = document.getElementById('my');
const ctx = canvas.getContext('2d');
const data = decode(u8);

const img = new ImageData(data.width, data.height);
const d = data.data;
for (let i = 0; i < data.width * data.height; i++) {
    img.data[i * 4 + 0] = d[i * 4 + 0];
    img.data[i * 4 + 1] = d[i * 4 + 1];
    img.data[i * 4 + 2] = d[i * 4 + 2];
    img.data[i * 4 + 3] = d[i * 4 + 3];
}
console.log(img);
ctx.putImageData(img, 0, 0);

let color_map =  new Uint16Array(2);
let width = 6;
let height = 6;
let currData = new Uint8Array(width * height * 12);
let frames = new Uint16Array(2);
console.log(encode(width, height, currData, frames, color_map, 1))