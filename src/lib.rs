mod utils;
#[macro_use]
extern crate serde_derive;

extern crate gif;
use gif::SetParameter;
use gif::{ColorOutput, Decoder, Encoder, Frame, Reader, Repeat};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub enum GifPlayMode {
    REPEAT,
    SINGLE
}

#[derive(Serialize, Deserialize)]
struct GifData {
    pub frames: Vec<FrameData>,
    pub global_palette: Vec<u8>,
    pub width: u16,
    pub height: u16
    
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
pub struct DecodeFrames {
    pub width: u16,
    pub height: u16,
    pub frameCount: u16,
    delays: Vec<u16>,
    palette: Vec<u8>,
    data: Vec<Vec<u8>>,
}

#[wasm_bindgen]
impl DecodeFrames {
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Box<[u8]> {
        let mut data = Vec::new();
        for frame in &self.data {
            data.extend(frame);
        }
        data.into_boxed_slice()
    }

    #[wasm_bindgen(getter)]
    pub fn delays(&self) -> Vec<u16> {
        self.delays.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn palette(&self) -> Vec<u8> {
        self.palette.clone()
    }
}

#[wasm_bindgen]
pub fn decode(data: &[u8]) -> DecodeFrames {
    let mut res: Vec<Vec<u8>> = Vec::new();
    let mut decoder = Decoder::new(data);
    let mut full_frame: Vec<u8> = Vec::new();

    decoder.set(ColorOutput::RGBA);

    let mut reader = decoder.read_info().unwrap();
    let (width, height, palette) = get_metadata(&reader);
    full_frame.resize((width as usize) * (height as usize) * (4 as usize), 0);

    let mut frame_count = 0;
    let mut delays: Vec<u16> = Vec::new();

    while let Some(frame) = reader.read_next_frame().unwrap() {
        let left = frame.left as usize;
        let top = frame.top as usize;
        let frame_width = frame.width as usize;
        let constant_offset = top * (width as usize) + left;
        for (i, pixel) in frame.buffer.chunks(4).enumerate() {
            if pixel.len() == 4 && pixel[3] != 0 {
                let index = constant_offset + (i / frame_width) * (width as usize) + (i % frame_width);
                full_frame[index * 4 + 0] = pixel[0];
                full_frame[index * 4 + 1] = pixel[1];
                full_frame[index * 4 + 2] = pixel[2];
                full_frame[index * 4 + 3] = pixel[3];
            }
        }
        frame_count += 1;
        res.push(full_frame.clone());
        delays.push(frame.delay);
    }
    DecodeFrames {
        width,
        height,
        frameCount: frame_count,
        palette,
        delays,
        data: res
    }
}

#[wasm_bindgen]
pub fn encode(width: u16, height: u16, frames: &mut [u8], delays: &[u16], global_palette: &[u8], mode: GifPlayMode, times: Option<u16>) -> Vec<u8> {
    let mut buffer = Vec::new();
    
    let mut encoder = Encoder::new(&mut buffer, width, height, global_palette).unwrap();
    match mode {
        GifPlayMode::REPEAT => encoder.set(Repeat::Infinite),
        GifPlayMode::SINGLE => encoder.set(Repeat::Finite(times.unwrap_or(1)))
    }.unwrap();

    for (i, delay) in delays.iter().enumerate() {
        let temp_data = &mut frames[i * (width as usize * height as usize * 4)..(i + 1) * (width as usize * height as usize * 4)];
        let mut temp = Frame::from_rgba(width, height, temp_data);
        temp.delay = *delay;
        encoder.write_frame(&temp).unwrap();
    }
    drop(encoder);
    buffer
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
