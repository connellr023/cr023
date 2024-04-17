use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern
{
    #[wasm_bindgen(js_name = setTimeout)]
    pub fn set_timeout(closure: &Closure<dyn FnMut()>, delay: u32) -> f64;
}