# graphacean

ferric grapheme

To build, run

```sh
wasm-pack build --target web
```

then do `python -m http.server` or something to host a local server to test the wasm thing.

For a new file, do

```rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WhateverYouWantPublicToJS {
    // ...
}

#[wasm_bindgen]
impl WhateverYouWantPublicToJS {
    // ...
}
```
