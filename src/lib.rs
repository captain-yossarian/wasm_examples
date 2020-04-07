extern crate serde_json;
extern crate wasm_bindgen;

extern crate js_sys;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde_derive;

#[wasm_bindgen]
extern "C" {

    pub type Quacks;

    #[wasm_bindgen(structural, method)]
    pub fn quack(this: &Quacks) -> Vec<i32>;

    #[wasm_bindgen(method, getter, structural)]
    pub fn id(this: &Quacks) -> i32;

}

#[derive(Serialize, Deserialize)]
pub struct Duck {
    id: String,
    name: String,
}

/// Next, we can export a function that takes any object that quacks:
#[wasm_bindgen]
pub fn quack(duck: &Quacks) -> Vec<i32> {
    duck.quack()
}

#[wasm_bindgen]
pub fn get_id(duck: &Quacks) -> i32 {
    duck.id()
}

#[wasm_bindgen]
pub fn filter_array_of_objects(some_iterable: &JsValue) -> Result<js_sys::Array, JsValue> {
    let ducks = js_sys::Array::new();

    let iterator =
        js_sys::try_iter(some_iterable)?.ok_or_else(|| "need to pass iterable JS values!")?;

    for duck in iterator {

        let entity = duck?;
        let elem: Duck = entity.into_serde().unwrap();
        if elem.id.parse::<i32>().unwrap_or(0) % 42 == 0 {
            let value = JsValue::from_serde(&elem).unwrap();
            ducks.push(&value);
        }
    }

    Ok(ducks)
}
