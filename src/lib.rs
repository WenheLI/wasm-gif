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
            rgba: full_frame.clone(),
            delay: frame.delay,
            width: frame.width,
            height: frame.height
        };
        res.push(frame)
    }
    let gif = GifData {
        width,
        height,
        global_palette: palette,
        frames: res
    };

    JsValue::from_serde(&gif).unwrap()
    
}

#[wasm_bindgen]
pub fn encode(data: &JsValue, mode: GifPlayMode, times: Option<u16>) -> Vec<u8> {
    let frames: GifData = data.into_serde().unwrap();
    let mut buffer = Vec::new();
    
    {
        let mut encoder = Encoder::new(&mut buffer, frames.width, frames.height, &frames.global_palette).unwrap();
        match mode {
            GifPlayMode::REPEAT => encoder.set(Repeat::Infinite),
            GifPlayMode::SINGLE => encoder.set(Repeat::Finite(times.unwrap_or(1)))
        }.unwrap();
        for frame in frames.frames.iter() {
            let mut temp_frame = Frame::from_rgba(frames.width, frames.height, &mut frame.rgba.to_vec());
            temp_frame.delay = frame.delay;
            encoder.write_frame(&temp_frame).unwrap();
        }
    }

    buffer
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
