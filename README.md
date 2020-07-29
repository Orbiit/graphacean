# graphacean

ferric grapheme

To build, run

```sh
wasm-pack build --target web
```

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
