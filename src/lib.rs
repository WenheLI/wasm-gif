mod utils;
#[macro_use]
extern crate serde_derive;

extern crate gif;
use gif::SetParameter;
use gif::{ColorOutput, Decoder, Encoder, Frame, Reader, Repeat};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Dimension {
    pub width: u16,
    pub heihgt: u16
}

#[derive(Serialize, Deserialize)]
struct FrameData {
    pub width: u16,
    pub height: u16,
    pub rgba: Vec<u8>,
    pub delay: u16
}

fn get_metadata(reader: &Reader<&[u8]>) -> (u16, u16, Vec<u8>) {
    let width = reader.width();
    let height = reader.height();
    let palette = reader.global_palette().unwrap_or(&Vec::new()).to_vec();
    (width, height, palette)
}

#[wasm_bindgen]
pub fn decode(data: &[u8]) -> JsValue {
    let mut res: Vec<FrameData> = Vec::new();
    let mut decoder = Decoder::new(data);
    let mut full_frame: Vec<u8> = Vec::new();
    decoder.set(ColorOutput::RGBA);
    let mut reader = decoder.read_info().unwrap();
    let (width, height, palette) = get_metadata(&reader);
    full_frame.resize((width as usize) * (height as usize) * (4 as usize), 0);

    while let Some(frame) = reader.read_next_frame().unwrap() {
        let buffer = frame.buffer.to_vec();
        let left = frame.left as usize;
        let top = frame.top as usize;
        let frame_width = frame.width as usize;
        let constant_offset = top * (width as usize) + left;
        for (i, pixel) in buffer.chunks(4).enumerate() {
            if pixel.len() == 4 && pixel[3] != 0 {
                let index = constant_offset + (i / frame_width) * (width as usize) + (i % frame_width);
                full_frame[index * 4 + 0] = pixel[0];
                full_frame[index * 4 + 1] = pixel[1];
                full_frame[index * 4 + 2] = pixel[2];
                full_frame[index * 4 + 3] = pixel[3];
            }
        }
        let frame = FrameData {
            width,
            height,
            rgba: full_frame.clone(),
            delay: frame.delay,
        };
        res.push(frame)
    }
    JsValue::from_serde(&res).unwrap()
    
}

// #[wasm_bindgen]
// pub fn encode(data: &JsValue) {
//     let frames: Vec<FrameData> = data.into_serde().unwrap();

// }

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;



#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wams-gif!");
}
