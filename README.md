# WASM-GIF-codec (WIP)

## Brief
This library offers an open-to-use library that allows developers to decode and encode gifs without any pain.

This library is based on `Rust`, `WASM`, and `Wasm-pack`.

## Usage
Currently, this library only supports `encoding` & `decoding` with little customization. In the following development, gif/image related operations and stream-based workflow will be introduced.

### Encode
```js
    import * as wasm from "wasm-gif";

    // Fetch a gif online first
    let req = await fetch("https://upload.wikimedia.org/wikipedia/commons/a/aa/SmallFullColourGIF.gif");
    // get the `uint8 array` from the request
    let ab = await req.arrayBuffer();
    let u8 = new Uint8Array(ab);

    const res = wasm.decode(u8);
```

### Decode
```js
    // res is an unpacked gif, encoded in Js Object format
    const data = wasm.encode(res, wasm.GifPlayMode.REPEAT, 1)
    // convert binary to blob object
    const url = URL.createObjectURL(new Blob([data], {type:"image/gif"}));
    console.log(url)
```
### Object Structure
Due to the limitation of `wasm_bindgen` complex objects can not directly get casted into `TypeScript` object. Therefore, it returns `any` in the function signature. I will explicitly post the structure here, until a solution is discovered.
```ts

class GifData {
    public frame: Array<FrameData>,
    public global_palette: Uint8Array,
    public width: number,
    public height: number
}

class FrameData {
    public width: number,
    public height: number,
    public rgba: Uint8Array,
    public delay: number
}
```