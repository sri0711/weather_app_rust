use gloo::console::log;
use wasm_bindgen::prelude::*;
use web_sys::{Geolocation, GeolocationPositionError};

#[wasm_bindgen]
pub fn get_locations() {
    let _geolocation = web_sys::window().unwrap().navigator().geolocation();
    let _success = Closure::wrap(Box::new(move |position: web_sys::Geolocation| {
        log!(format!("{:?}", position));
    }) as Box<dyn FnMut(web_sys::Geolocation)>);
}
